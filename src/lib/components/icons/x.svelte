<script lang="ts">
	import { createIconAnimation, type IconProps } from './shared.svelte';

	let {
		color = 'currentColor',
		size = 24,
		strokeWidth = 2,
		class: className = '',
		animate: external
	}: IconProps = $props();

	const anim = createIconAnimation('timeout', 500);
	const isActive = $derived(external ?? anim.animate);
</script>

<div class={className} aria-label="close" role="img" onmouseenter={anim.onEnter}>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width={size}
		height={size}
		viewBox="0 0 24 24"
		fill="none"
		stroke={color}
		stroke-width={strokeWidth}
		stroke-linecap="round"
		stroke-linejoin="round"
		class="x"
		class:animate={isActive}
		aria-hidden="true"
	>
		<title>close</title>
		<path d="M18 6 6 18" class="diagonal-1" />
		<path d="m6 6 12 12" class="diagonal-2" />
	</svg>
</div>

<style>
	div {
		display: inline-block;
	}
	.x {
		overflow: visible;
	}

	.diagonal-1,
	.diagonal-2 {
		stroke-dasharray: 17;
		stroke-dashoffset: 0;
		transition: stroke-dashoffset 0.15s ease-out;
	}

	.x.animate .diagonal-1 {
		opacity: 0;
		animation: lineAnimation 0.3s ease-out forwards;
	}

	.x.animate .diagonal-2 {
		opacity: 0;
		animation: lineAnimation 0.3s ease-out 0.25s forwards;
	}

	@keyframes lineAnimation {
		0% {
			opacity: 0;
			stroke-dashoffset: 17;
		}
		15% {
			opacity: 1;
			stroke-dashoffset: 17;
		}
		100% {
			opacity: 1;
			stroke-dashoffset: 0;
		}
	}
</style>
