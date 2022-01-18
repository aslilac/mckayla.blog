module.exports = {
	rewrites: async () => [
		{
			source: "/:date/:title",
			destination: "/:title",
		},
	],
};
