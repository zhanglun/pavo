<script lang="ts">
	import { createIconAnimation, type IconProps } from './shared.svelte';

	let {
		color = 'currentColor',
		size = 24,
		strokeWidth = 2,
		class: className = '',
		animate: external
	}: IconProps = $props();

	const anim = createIconAnimation('timeout', 200);
	const isActive = $derived(external ?? anim.animate);
</script>

<div class={className} aria-label="chevron-right" role="img" onmouseenter={anim.onEnter}>
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
		aria-hidden="true"
	>
		<title>chevron-right</title>
		<path d="m9 18 6-6-6-6" class:path-nudge={isActive} />
	</svg>
</div>

<style>
	div {
		display: inline-block;
	}
	path {
		transition: all 0.2s ease-in;
	}

	.path-nudge {
		transform: translateX(3px);
	}
</style>
