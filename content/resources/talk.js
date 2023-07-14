document.body.addEventListener(
	"keydown",
	(event) => {
		switch (event.key) {
			case "k":
			case "ArrowUp":
			case "ArrowLeft":
				event.preventDefault();
				goToPrevious();
				break;
			case "j":
			case "ArrowDown":
			case "ArrowRight":
			case " ":
				event.preventDefault();
				goToNext();
				break;
		}
	},
	{ passive: false },
);

window.addEventListener("click", goToNext);

const statePrefix = "slide-";
function getCurrentSlideIndex() {
	const state = location.hash.substring(1);

	let index = state.startsWith(statePrefix)
		? parseInt(state.substring(statePrefix.length))
		: 0;

	if (isNaN(index)) {
		index = 0;
	}

	return index;
}

function getLastSlideIndex() {
	const slides = [...document.querySelectorAll("[data-slide]")];
	return Math.max(
		...slides
			.filter((slide) => slide.id.startsWith(statePrefix))
			.map((slide) => parseInt(slide.id.substring(statePrefix.length))),
	);
}

function goToPrevious() {
	const currentSlide = getCurrentSlideIndex();
	location.hash = "";
	location.hash = statePrefix + Math.max(0, currentSlide - 1);
}

function goToNext() {
	const currentSlide = getCurrentSlideIndex();
	location.hash = "";
	location.hash = statePrefix + Math.min(currentSlide + 1, getLastSlideIndex());
}
