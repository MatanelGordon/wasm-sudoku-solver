<script setup lang="ts">
	import type { ComponentEvent } from '@/types';
	import type { CellEventPayload } from './types';
	import { computed } from 'vue';

	const emit = defineEmits(['focus', 'click']);
	type CellEvent = ComponentEvent<typeof emit>;

	const props = defineProps({
		value: {
			type: Number,
			default: 0
		},
		row: {
			type: Number,
			required: true
		},
		col: {
			type: Number,
			required: true
		},
		size: {
			type: Number,
			default: 0
		},
		className: String,
		selected: {
			type: Boolean,
			default: false
		}
	});

	const tab_index = computed(() => props.row * props.size + props.col + 1);

	function createEmitter<Evt extends Event>(name: CellEvent) {
		return (evt: Evt) => {
			const payload: CellEventPayload<Evt> = {
				row: props.row,
				col: props.col,
				selected: props.selected,
				event: evt
			};

			emit(name, payload);
		};
	}

	const focusHandler = createEmitter('focus');
	const clickHandler = createEmitter('click');
</script>

<template>
	<button
		data-cell="true"
		class="cell"
		:class="[props.className, { active: props.selected }]"
		:tabindex="tab_index"
		@focus.prevent.stop="focusHandler"
		@mousedown.prevent.stop="clickHandler"
	>
		{{ props.value > 0 ? props.value.toString() : '' }}
	</button>
</template>

<style scoped lang="less">
	.cell {
		cursor: pointer;
		outline: none;
		background: none;
		font-weight: 700;
		border: thin solid #ccc;
		transition: background-color ease-out 0.1s;

		&.active {
			background-color: var(--selected-color, indianred);
			color: var(--selected-color-text, var(--color-dark));
		}

		&:not(.active) {
			&::after {
				content: '';
				block-size: 100%;
				inline-size: 100%;
				position: absolute;
				inset: 0;
				background-color: var(--selected-color, indianred);
				opacity: 0;
			}

			&:hover:after {
				opacity: 0.6;
			}
		}
	}
</style>
