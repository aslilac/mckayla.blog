Prism.languages.gleam = {
	doc: {
		pattern: /\/\/\/\/?.*/,
		greedy: true,
	},
	comment: {
		pattern: /\/\/.*/,
		greedy: true,
	},
	function: /([a-zA-Z_][a-zA-Z0-9_]+)(?=\()/,
	keyword:
		/\b(use|case|if|external|fn|import|let|assert|try|pub|type|opaque|const|todo|as)\b/,
	operator: {
		pattern:
			/(<<|>>|<-|->|\|>|<>|\.\.|<=\.?|>=\.?|==\.?|!=\.?|<\.?|>\.?|&&|\|\||\+\.?|-\.?|\/\.?|\*\.?|%\.?|=)/,
		greedy: true,
	},
	string: {
		pattern: /"(?:\\(?:\r\n|[\s\S])|(?!")[^\\\r\n])*"/,
		greedy: true,
		inside: {
			punctuation: /\\./,
		},
	},
	punctuation: /[.\\:,{}()]/,
	number:
		/\b(?:0b[0-1]+|0o[0-7]+|[[:digit:]][[:digit:]_]*(\\.[[:digit:]]*)?|0x[[:xdigit:]]+)\b/,
	boolean: /\b(?:True|False|Int|Float|String)\b/,
};
