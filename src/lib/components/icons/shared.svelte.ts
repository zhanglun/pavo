export interface IconProps {
	color?: string;
	size?: number;
	strokeWidth?: number;
	class?: string;
	animate?: boolean;
}

/** toggle = 跟随鼠标进出切换；timeout = 触发后定时自动复位 */
type AnimationMode = 'toggle' | 'timeout';

/** 返回对象通过 getter 暴露响应式状态，调用方须通过属性访问保持响应性，不可解构 */
export function createIconAnimation(mode: AnimationMode, duration = 300) {
	let animate = $state(false);
	let timer: ReturnType<typeof setTimeout> | undefined;

	function clearTimer() {
		if (timer !== undefined) {
			clearTimeout(timer);
			timer = undefined;
		}
	}

	if (mode === 'timeout') {
		return {
			get animate() {
				return animate;
			},
			onEnter() {
				if (animate) return;
				animate = true;
				clearTimer();
				timer = setTimeout(() => {
					animate = false;
				}, duration);
			},
		};
	}

	return {
		get animate() {
			return animate;
		},
		onEnter() {
			animate = true;
		},
		onLeave() {
			animate = false;
		},
	};
}
