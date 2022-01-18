import DocumentComponent, {
	DocumentContext,
	Head,
	Html,
	Main,
	NextScript,
} from "next/document";

class Document extends DocumentComponent {
	static async getInitialProps(ctx: DocumentContext) {
		return await DocumentComponent.getInitialProps(ctx);
	}

	render() {
		return (
			<Html>
				<Head>
					<link rel="shortcut icon" href="/favicon.png" />
				</Head>
				<body>
					<Main />
					<NextScript />
				</body>
			</Html>
		);
	}
}

export default Document;
