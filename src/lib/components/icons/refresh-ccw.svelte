<script lang="ts">
	import { createIconAnimation, type IconProps } from './shared.svelte';

	let {
		color = 'currentColor',
		size = 24,
		strokeWidth = 2,
		class: className = '',
		animate: external
	}: IconProps = $props();

	const anim = createIconAnimation('toggle');
	const isActive = $derived(external ?? anim.animate);
</script>

<div
	class={className}
	aria-label="refresh"
	role="img"
	onmouseenter={anim.onEnter}
	onmouseleave={anim.onLeave}
>
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
		class="refresh-icon"
		class:animate={isActive}
		aria-hidden="true"
	>
		<title>refresh</title>
		<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
		<path d="M3 3v5h5" />
		<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
		<path d="M16 16h5v5" />
	</svg>
</div>

<style>
	div {
		display: inline-block;
	}
	.refresh-icon {
		transition: transform 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
		transform-origin: center;
	}

	.refresh-icon.animate {
		transform: rotate(-50deg);
	}

	svg {
		overflow: visible;
	}
</style>
