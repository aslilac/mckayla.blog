use std::path::PathBuf;
use std::process::exit;

#[derive(Clone, Debug, Default)]
struct OptionsBuilder {
	output: Option<PathBuf>,
	publish: bool,
}

#[derive(Clone, Debug)]
pub struct Options {
	pub output: PathBuf,
	pub publish: bool,
}

impl From<OptionsBuilder> for Options {
	fn from(builder: OptionsBuilder) -> Self {
		Options {
			output: builder.output.unwrap_or_else(|| PathBuf::from("./output/")),
			publish: builder.publish,
		}
	}
}

impl<S> FromIterator<S> for Options
where
	S: AsRef<str>,
{
	fn from_iter<I>(args: I) -> Self
	where
		I: IntoIterator<Item = S>,
	{
		let mut options = OptionsBuilder::default();
		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let arg = arg.as_ref();
			if (arg.len() >= 2 && arg.starts_with('-')) || arg.len() >= 3 && arg.starts_with("--") {
				match arg {
					"-p" | "-pub" | "--pub" | "-publish" | "--publish" => {
						options.publish = true;
					}
					_ => {
						println!("unrecognized option: {}", arg);
						exit(1);
					}
				}
			} else {
				options.output = Some(PathBuf::from(arg));
			}
		}

		options.into()
	}
}
