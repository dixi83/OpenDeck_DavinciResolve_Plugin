use std::sync::LazyLock;
use std::time::Instant;
use std::collections::HashMap;
use std::process::Command;
use std::path::PathBuf;

use enigo::{Direction, Enigo, Key, Settings, agent::{Agent, Token}};
use openaction::*;
use tokio::sync::Mutex;

static ENIGO: LazyLock<Mutex<Option<Enigo>>> = LazyLock::new(|| Mutex::new(None));
static TIMELINE_JOG_TIMES: LazyLock<Mutex<HashMap<String, Instant>>> =
	LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct ActionSettings {}

async fn press(tokens: &[Token]) {
	let mut guard = ENIGO.lock().await;
	if guard.is_none() {
		match Enigo::new(&Settings::default()) {
			Ok(enigo) => { guard.replace(enigo); }
			Err(error) => {
				log::error!("Could not initialize keyboard input: {error}");
				return;
			}
		}
	}
	if let Some(enigo) = guard.as_mut() {
		for token in tokens {
			if let Err(error) = enigo.execute(token) {
				log::error!("Could not send keyboard input: {error}");
				break;
			}
		}
	}
}

async fn accelerated_steps(instance_id: &str, ticks: i16) -> usize {
	if ticks == 0 {
		return 0;
	}
	let now = Instant::now();
	let last = TIMELINE_JOG_TIMES
		.lock()
		.await
		.insert(instance_id.to_owned(), now);
	let tick_count = ticks.unsigned_abs().max(1) as u128;
	let ms_per_tick = last
		.map(|last| now.duration_since(last).as_millis() / tick_count)
		.unwrap_or(250);
	let multiplier = match ms_per_tick {
		0..=39 => 8,
		40..=79 => 4,
		80..=159 => 2,
		_ => 1,
	};
	(tick_count as usize).saturating_mul(multiplier).min(120)
}

async fn open_resolve_page(page: &'static str) {
	let result = tokio::task::spawn_blocking(move || {
		let program_files = std::env::var_os("ProgramFiles")
			.map(PathBuf::from)
			.unwrap_or_else(|| PathBuf::from(r"C:\Program Files"));
		let interpreter = program_files.join(r"Blackmagic Design\DaVinci Resolve\ResolvePython\ResolvePython.exe");
		let script = format!(
			"import DaVinciResolveScript as dvr; r=dvr.scriptapp('Resolve'); print(r.OpenPage('{page}') if r else 'NO_RESOLVE')"
		);
		Command::new(interpreter).args(["-c", script.as_str()]).output()
	})
	.await;

	match result {
		Ok(Ok(output)) if output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "True" => {}
		Ok(Ok(output)) => log::error!(
			"Could not open Resolve page {page}: {}{}",
			String::from_utf8_lossy(&output.stdout).trim(),
			String::from_utf8_lossy(&output.stderr).trim()
		),
		Ok(Err(error)) => log::error!("Could not launch Resolve scripting API: {error}"),
		Err(error) => log::error!("Resolve page switch task failed: {error}"),
	}
}

macro_rules! control_action {
	($name:ident, $uuid:literal, $($token:expr),+ $(,)?) => {
		struct $name;
		#[async_trait]
		impl Action for $name {
			const UUID: &'static str = $uuid;
			type Settings = ActionSettings;
			async fn key_up(&self, _instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
				press(&[$($token),+]).await;
				Ok(())
			}
			async fn dial_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
				self.key_up(instance, settings).await
			}
		}
	};
}

macro_rules! page_action {
	($name:ident, $uuid:literal, $page:literal) => {
		struct $name;
		#[async_trait]
		impl Action for $name {
			const UUID: &'static str = $uuid;
			type Settings = ActionSettings;
			async fn key_up(&self, _instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
				open_resolve_page($page).await;
				Ok(())
			}
			async fn dial_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
				self.key_up(instance, settings).await
			}
		}
	};
}

page_action!(EditPage, "com.martijn.opendeck.davinciresolve.editpage", "edit");
page_action!(MediaPage, "com.martijn.opendeck.davinciresolve.mediapage", "media");
page_action!(PhotoPage, "com.martijn.opendeck.davinciresolve.photopage", "photo");
page_action!(CutPage, "com.martijn.opendeck.davinciresolve.cutpage", "cut");
page_action!(FusionPage, "com.martijn.opendeck.davinciresolve.fusionpage", "fusion");
page_action!(ColorPage, "com.martijn.opendeck.davinciresolve.colorpage", "color");
page_action!(FairlightPage, "com.martijn.opendeck.davinciresolve.fairlightpage", "fairlight");
page_action!(DeliverPage, "com.martijn.opendeck.davinciresolve.deliverpage", "deliver");
control_action!(PlayPause, "com.martijn.opendeck.davinciresolve.playpause", Token::Key(Key::Space, Direction::Click));

struct TimelineJog;
#[async_trait]
impl Action for TimelineJog {
	const UUID: &'static str = "com.martijn.opendeck.davinciresolve.timelinejog";
	type Settings = ActionSettings;

	async fn dial_rotate(
		&self,
		instance: &Instance,
		_ticks_settings: &Self::Settings,
		ticks: i16,
		_pressed: bool,
	) -> OpenActionResult<()> {
		let frames = accelerated_steps(&instance.instance_id, ticks).await;
		if frames == 0 {
			return Ok(());
		}
		let key = if ticks < 0 { Key::LeftArrow } else { Key::RightArrow };
		let mut tokens = vec![
			Token::Key(Key::Control, Direction::Press),
			Token::Key(Key::Num4, Direction::Click),
			Token::Key(Key::Control, Direction::Release),
		];
		tokens.extend(vec![Token::Key(key, Direction::Click); frames]);
		press(&tokens).await;
		Ok(())
	}

	async fn dial_up(&self, _instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
		press(&[Token::Key(Key::Space, Direction::Click)]).await;
		Ok(())
	}

	async fn will_disappear(&self, instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
		TIMELINE_JOG_TIMES.lock().await.remove(&instance.instance_id);
		Ok(())
	}
}

struct SourceClipJog;
#[async_trait]
impl Action for SourceClipJog {
	const UUID: &'static str = "com.martijn.opendeck.davinciresolve.sourceclipjog";
	type Settings = ActionSettings;

	async fn dial_rotate(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
		ticks: i16,
		_pressed: bool,
	) -> OpenActionResult<()> {
		let frames = accelerated_steps(&instance.instance_id, ticks).await;
		if frames == 0 {
			return Ok(());
		}
		let key = if ticks < 0 { Key::LeftArrow } else { Key::RightArrow };
		let mut tokens = vec![
			Token::Key(Key::Control, Direction::Press),
			Token::Key(Key::Num3, Direction::Click),
			Token::Key(Key::Control, Direction::Release),
		];
		tokens.extend(vec![Token::Key(key, Direction::Click); frames]);
		press(&tokens).await;
		Ok(())
	}

	async fn dial_up(&self, _instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
		press(&[Token::Key(Key::Space, Direction::Click)]).await;
		Ok(())
	}

	async fn will_disappear(&self, instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
		TIMELINE_JOG_TIMES.lock().await.remove(&instance.instance_id);
		Ok(())
	}
}

struct TimelineZoom;
#[async_trait]
impl Action for TimelineZoom {
	const UUID: &'static str = "com.martijn.opendeck.davinciresolve.timelinezoom";
	type Settings = ActionSettings;

	async fn dial_rotate(
		&self,
		_instance: &Instance,
		_settings: &Self::Settings,
		ticks: i16,
		_pressed: bool,
	) -> OpenActionResult<()> {
		if ticks == 0 { return Ok(()); }
		let key = if ticks < 0 { Key::OEMMinus } else { Key::OEMPlus };
		let mut tokens = vec![
			Token::Key(Key::Control, Direction::Press),
			Token::Key(Key::Num4, Direction::Click),
			Token::Key(Key::Control, Direction::Release),
			Token::Key(Key::Control, Direction::Press),
		];
		tokens.extend(vec![Token::Key(key, Direction::Click); ticks.unsigned_abs().min(12) as usize]);
		tokens.push(Token::Key(Key::Control, Direction::Release));
		press(&tokens).await;
		Ok(())
	}

	async fn dial_up(&self, _instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
		press(&[
			Token::Key(Key::Control, Direction::Press),
			Token::Key(Key::Num4, Direction::Click),
			Token::Key(Key::Control, Direction::Release),
			Token::Key(Key::Shift, Direction::Press),
			Token::Key(Key::Z, Direction::Click),
			Token::Key(Key::Shift, Direction::Release),
		]).await;
		Ok(())
	}
}

control_action!(PlayReverse, "com.martijn.opendeck.davinciresolve.playreverse", Token::Key(Key::J, Direction::Click));
control_action!(StopPlayback, "com.martijn.opendeck.davinciresolve.stop", Token::Key(Key::K, Direction::Click));
control_action!(PlayForward, "com.martijn.opendeck.davinciresolve.playforward", Token::Key(Key::L, Direction::Click));
control_action!(PreviousFrame, "com.martijn.opendeck.davinciresolve.previousframe", Token::Key(Key::LeftArrow, Direction::Click));
control_action!(NextFrame, "com.martijn.opendeck.davinciresolve.nextframe", Token::Key(Key::RightArrow, Direction::Click));
control_action!(PreviousEdit, "com.martijn.opendeck.davinciresolve.previousedit", Token::Key(Key::UpArrow, Direction::Click));
control_action!(NextEdit, "com.martijn.opendeck.davinciresolve.nextedit", Token::Key(Key::DownArrow, Direction::Click));
control_action!(MarkIn, "com.martijn.opendeck.davinciresolve.markin", Token::Key(Key::I, Direction::Click));
control_action!(MarkOut, "com.martijn.opendeck.davinciresolve.markout", Token::Key(Key::O, Direction::Click));
control_action!(AddMarker, "com.martijn.opendeck.davinciresolve.addmarker", Token::Key(Key::M, Direction::Click));
control_action!(InsertClip, "com.martijn.opendeck.davinciresolve.insertclip", Token::Key(Key::F9, Direction::Click));
control_action!(SplitClip, "com.martijn.opendeck.davinciresolve.splitclip",
	Token::Key(Key::Control, Direction::Press), Token::Key(Key::B, Direction::Click), Token::Key(Key::Control, Direction::Release));
control_action!(Undo, "com.martijn.opendeck.davinciresolve.undo",
	Token::Key(Key::Control, Direction::Press), Token::Key(Key::Z, Direction::Click), Token::Key(Key::Control, Direction::Release));
control_action!(Redo, "com.martijn.opendeck.davinciresolve.redo",
	Token::Key(Key::Control, Direction::Press), Token::Key(Key::Shift, Direction::Press), Token::Key(Key::Z, Direction::Click),
	Token::Key(Key::Shift, Direction::Release), Token::Key(Key::Control, Direction::Release));
control_action!(SelectionTool, "com.martijn.opendeck.davinciresolve.selectiontool", Token::Key(Key::A, Direction::Click));
control_action!(BladeTool, "com.martijn.opendeck.davinciresolve.bladetool", Token::Key(Key::B, Direction::Click));

#[tokio::main]
async fn main() -> OpenActionResult<()> {
	let _ = simplelog::TermLogger::init(
		log::LevelFilter::Info,
		simplelog::Config::default(),
		simplelog::TerminalMode::Stdout,
		simplelog::ColorChoice::Never,
	);
	register_action(EditPage).await;
	register_action(MediaPage).await;
	register_action(PhotoPage).await;
	register_action(CutPage).await;
	register_action(FusionPage).await;
	register_action(ColorPage).await;
	register_action(FairlightPage).await;
	register_action(DeliverPage).await;
	register_action(PlayPause).await;
	register_action(TimelineJog).await;
	register_action(SourceClipJog).await;
	register_action(TimelineZoom).await;
	register_action(PlayReverse).await;
	register_action(StopPlayback).await;
	register_action(PlayForward).await;
	register_action(PreviousFrame).await;
	register_action(NextFrame).await;
	register_action(PreviousEdit).await;
	register_action(NextEdit).await;
	register_action(MarkIn).await;
	register_action(MarkOut).await;
	register_action(AddMarker).await;
	register_action(InsertClip).await;
	register_action(SplitClip).await;
	register_action(Undo).await;
	register_action(Redo).await;
	register_action(SelectionTool).await;
	register_action(BladeTool).await;
	run(std::env::args().collect()).await
}
