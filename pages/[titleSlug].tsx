import type { GetStaticPaths, GetStaticProps } from "next";
import { is, $object, $string, matches, Type } from "succulent";

export const getStaticPaths: GetStaticPaths = async () => {
	const fs = await import("fs/promises");

	return {
		paths: [{ params: { titleSlug: "example-post" } }],
		fallback: false,
	};
};

const $BlogPostData = $object({
	title: $string,
	// date: $string.that(matches(/\d{1,2}\.\d{1,2}\d\.\d{1,2}/)),
	content: $string,
});

interface BlogPostProps extends Type<typeof $BlogPostData> {}

export const getStaticProps: GetStaticProps<BlogPostProps> = async (ctx) => {
	const fs = await import("fs/promises");
	const path = await import("path");

	const postData = JSON.parse(
		await fs.readFile(
			path.join(process.cwd(), `./posts/${ctx.params!["titleSlug"]}.json`),
			"utf-8",
		),
	);

	if (!is(postData, $BlogPostData)) {
		throw new Error("Invalid post data");
	}

	return {
		props: {
			...postData,
		},
	};
};

const BlogPost = (props: BlogPostProps) => {
	const { title, content } = props;

	return (
		<div>
			<h1>{title}</h1>
			<p>{content}</p>
		</div>
	);
};

export default BlogPost;
