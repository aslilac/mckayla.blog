use std::process::exit;

#[derive(Clone, Debug, Default)]
pub struct Options {
	pub publish: bool,
}

impl<S> FromIterator<S> for Options
where
	S: AsRef<str>,
{
	fn from_iter<I>(args: I) -> Self
	where
		I: IntoIterator<Item = S>,
	{
		let mut options = Options::default();
		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let arg = arg.as_ref();
			if (arg.len() == 2 && arg.starts_with('-')) || arg.len() > 3 && arg.starts_with("--") {
				match arg {
					"-publish" => {
						options.publish = true;
					}
					_ => {
						println!("unrecognized option: {}", arg);
						exit(1);
					}
				}
			} else {
				println!("unrecognized argument: {}", arg);
				exit(1);
			}
		}

		options
	}
}
