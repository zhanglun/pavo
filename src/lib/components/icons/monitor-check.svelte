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

<div class={className} aria-label="monitor-check" role="img" onmouseenter={anim.onEnter}>
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
		class="monitor-check-icon"
		class:animate={isActive}
		aria-hidden="true"
	>
		<title>monitor-check</title>
		<path d="m9 10 2 2 4-4" class="check-path" />
		<rect width="20" height="14" x="2" y="3" rx="2" />
		<path d="M12 17v4" />
		<path d="M8 21h8" />
	</svg>
</div>

<style>
	div {
		display: inline-block;
	}
	.monitor-check-icon {
		overflow: visible;
	}
	.check-path {
		stroke-dasharray: 9;
		stroke-dashoffset: 0;
		transition:
			stroke-dashoffset 0.125s ease-out,
			opacity 0.125s ease-out;
	}
	.monitor-check-icon.animate .check-path {
		animation: checkAnimation 0.5s ease-out backwards;
	}
	@keyframes checkAnimation {
		0% {
			stroke-dashoffset: 9;
			opacity: 0;
		}
		33% {
			stroke-dashoffset: 9;
			opacity: 0;
		}
		100% {
			stroke-dashoffset: 0;
			opacity: 1;
		}
	}
</style>
