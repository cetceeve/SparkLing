<script>
  import { onMount } from 'svelte';
  import "ol/ol.css";
  import Map from 'ol/Map.js';
  import OSM from 'ol/source/OSM.js';
  import TileLayer from 'ol/layer/Tile.js';
  import View from 'ol/View.js';

  export let map = undefined;

  onMount(() => {
    map = new Map({
      target: 'map-container',
      layers: [
        new TileLayer({
          source: new OSM(),
        }),
      ],
      view: new View({
        center: [0, 0],
        zoom: 2,
      }),
    });
  });
</script>

<div id="map-container"></div>

<style>

  #map-container {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 100%;
  }
</style>
