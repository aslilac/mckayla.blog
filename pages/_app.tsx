import type { AppProps } from "next/app";

import "../style/puppy.css";

export default function App(props: AppProps) {
	const { Component, pageProps } = props;
	return <Component {...pageProps} />;
}
