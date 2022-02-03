import type { GetStaticProps } from "next";
import Head from "next/head";
import Link from "next/link";
import React from "react";
import { is, Type, $object, $string } from "succulent";

const $BlogPost = $object({
	title: $string,
	titleSlug: $string,
});

interface HomeProps {
	posts: Array<Type<typeof $BlogPost>>;
}

// export const getStaticProps: GetStaticProps<HomeProps> = async () => {
// 	const fs = await import("fs/promises");
// 	const path = await import("path");
// 	const postFiles = await fs.readdir(path.join(process.cwd(), "./posts/"));

// 	return {
// 		props: { posts: postFiles.map((titleSlug) => ({ title: titleSlug, titleSlug })) },
// 	};
// };

const Lilac = (props: HomeProps) => {
	const { posts } = props;

	return (
		<>
			<Head>
				<title>puppy</title>
				<meta property="og:title" content="puppy" key="title" />
			</Head>
			<div className="timeline">
				{/* <div className="timeline-item">{posts.map((post) => <Link href={post.title.slice(0, -5)}><a>{post.title}</a></Link>)}</div> */}
			</div>
		</>
	);
};

export default Lilac;
