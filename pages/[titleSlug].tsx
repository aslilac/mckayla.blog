import type {
	GetPageResponse,
	GetBlockResponse,
	ListBlockChildrenResponse,
} from "@notionhq/client";
import type { GetStaticPaths, GetStaticProps } from "next";
import { is, $object, $string, matches, Type } from "succulent";

import { notion } from "^/base";

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

interface BlogPostProps {
	// page: Type<typeof $BlogPostData>;
	page: GetPageResponse;
	block: GetBlockResponse;
	blockItems: ListBlockChildrenResponse;
}

export const getStaticProps: GetStaticProps<BlogPostProps> = async (ctx) => {
	const page = await notion.pages.retrieve({
		page_id: "f9fa42ca-8d23-4f99-be60-e3f91af52a45",
	});

	const block = await notion.blocks.retrieve({
		block_id: "f9fa42ca-8d23-4f99-be60-e3f91af52a45",
	});

	const blockItems = await notion.blocks.children.list({
		block_id: "f9fa42ca-8d23-4f99-be60-e3f91af52a45",
	});

	return {
		props: {
			page,
			block,
			blockItems,
		},
	};
};

const BlogPost = (props: BlogPostProps) => {
	const { block, page, blockItems } = props;

	return (
		<div>
			<h1>Hello</h1>
			<img src={page.cover.file.url} width={400} />
			<h3>page</h3>
			<pre>{JSON.stringify(page, null, 2)}</pre>
			<h3>block</h3>
			<pre>{JSON.stringify(block, null, 2)}</pre>
			<h3>blockItems</h3>
			<pre>{JSON.stringify(blockItems, null, 2)}</pre>
		</div>
	);
};

export default BlogPost;
