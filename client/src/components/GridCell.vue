<script setup lang="ts">
	import { computed } from 'vue';

	const emit = defineEmits(['focus', 'click']);

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
			required: true
		},
		className: String,
		selected: {
			type: Boolean,
			default: false
		}
	});

	const tab_index = computed(() => props.row * props.size + props.col);
	const payload = computed(() => ({
		row: props.row,
		col: props.col,
		selected: props.selected
	}));
</script>

<template>
	<button
		class="cell"
		:class="[props.className, { active: props.selected }]"
		:tabindex="tab_index"
		@focus="emit('focus', payload)"
		@click="emit('click', payload)"
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
		border: thin solid var(--color);
		color: var(--color);

		&.active {
			background-color: var(--selected-color, indianred);
		}
	}
</style>
