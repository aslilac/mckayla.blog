import type { AppProps } from "next/app";

import "./pico.min.css";

export default function App(props: AppProps) {
	const { Component, pageProps } = props;
	return <Component {...pageProps} />;
}
