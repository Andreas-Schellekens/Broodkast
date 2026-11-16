<script lang="ts">
	/** Beeldmerk: broodkast met roldeksel en antenne, gevuld met een warmteverloop. */
	let { size = 44, label }: { size?: number; label?: string } = $props();
	const uid = $props.id();
	const gradId = `bk-hitte-${uid}`;
</script>

<svg
	width={size}
	height={Math.round((size * 82) / 104)}
	viewBox="0 0 104 82"
	role={label ? 'img' : undefined}
	aria-label={label}
	aria-hidden={label ? undefined : 'true'}
>
	<defs>
		<radialGradient id={gradId} cx=".35" cy=".75" r=".9">
			<stop offset="0" stop-color="#FFF3C4" />
			<stop offset=".18" stop-color="#FFD45A" />
			<stop offset=".42" stop-color="#FF6A2B" />
			<stop offset=".7" stop-color="#FF2E88" />
			<stop offset="1" stop-color="#7A2CF0" />
		</radialGradient>
	</defs>
	<path d="M80 20L88 6" stroke="#FFFFFF" stroke-width="2.8" stroke-linecap="round" />
	<circle cx="89" cy="5" r="3.8" fill="#FF2E88" />
	<path d="M95 1.5Q98.5 5 95 8.5" stroke="#FFFFFF" stroke-width="1.6" fill="none" stroke-linecap="round" />
	<rect x="30" y="19" width="62" height="6" rx="2" fill="#FF2E88" />
	<path d="M58 78V52C58 36 68 24 84 24H92V78Z" fill="#B0206A" />
	<path d="M8 78V52C8 36 18 24 34 24H84C68 24 58 36 58 52V78Z" fill="url(#{gradId})" />
	<path
		d="M9.2 42.9H59.2M12.5 35.2H62.5M17.9 29.3H67.9M8 52H58M8 60H58M8 68H58"
		stroke="#0B0806"
		stroke-opacity=".28"
		stroke-width="1.4"
	/>
</svg>
