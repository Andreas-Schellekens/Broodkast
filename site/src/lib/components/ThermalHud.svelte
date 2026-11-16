<script lang="ts">
	import NotificationCard from './NotificationCard.svelte';

	const schaal = ['230°', '160°', '90°', '20°'];
</script>

<!-- Warmtecamera-overlay: temperatuurschaal, richtkruis en twee voorbeeldmeldingen. -->
<div class="hud">
	<div class="schaal" aria-hidden="true">
		<div class="getallen">
			{#each schaal as t (t)}<span>{t}</span>{/each}
		</div>
		<div class="balk"></div>
	</div>
	<div class="kruis" aria-hidden="true">
		<span class="h"></span><span class="v"></span><span class="ring"></span>
		<span class="label">212,4 °C <small>korst</small></span>
	</div>
	<div class="meldingen" aria-label="Voorbeelden van meldingen">
		<NotificationCard bron="Twitch · nu live" titel="bakkerbram is live: nachtploeg, deel 3" meta="@streamkijkers · #live" live />
		<NotificationCard bron="Feed · YouTube" titel="Nieuwe video: zuurdesem in 24 uur" meta="#video's" />
	</div>
</div>

<style>
	.hud {
		position: relative;
		height: 460px;
	}
	.schaal {
		position: absolute;
		right: 0;
		top: 10px;
		height: 300px;
		display: flex;
		gap: 10px;
	}
	.getallen {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		text-align: right;
		font-family: var(--font-display);
		font-weight: 600;
		font-size: 15px;
		font-variant-numeric: tabular-nums;
	}
	.balk {
		width: 12px;
		border-radius: 3px;
		background: linear-gradient(180deg, #fff3c4, #ffd45a 12%, #ffa23a 30%, #ff5a3c 48%, #f02d8c 66%, #8c3cf2 84%, #4a63ff);
	}
	.kruis {
		position: absolute;
		left: 30%;
		top: 46%;
	}
	.h,
	.v,
	.ring {
		position: absolute;
		background: var(--text);
	}
	.h {
		left: -22px;
		width: 44px;
		height: 1px;
	}
	.v {
		top: -22px;
		width: 1px;
		height: 44px;
	}
	.ring {
		left: -9px;
		top: -9px;
		width: 18px;
		height: 18px;
		border: 1px solid var(--text);
		border-radius: 50%;
		background: none;
	}
	.label {
		position: absolute;
		left: 24px;
		top: -44px;
		white-space: nowrap;
		background: rgb(11 8 6 / 0.72);
		border: 1px solid var(--line-strong);
		border-radius: 4px;
		padding: 6px 10px;
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 22px;
		line-height: 1;
		font-variant-numeric: tabular-nums;
	}
	.label small {
		font-family: var(--font-body);
		font-weight: 500;
		font-size: 12px;
		color: var(--text-soft);
		margin-left: 4px;
	}
	.meldingen {
		position: absolute;
		left: 0;
		right: 40px;
		bottom: 0;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}
	@media (max-width: 900px) {
		.hud {
			height: auto;
		}
		.schaal,
		.kruis {
			display: none;
		}
		.meldingen {
			position: static;
		}
	}
</style>
