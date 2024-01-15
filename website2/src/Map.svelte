<script>
  import { onMount } from 'svelte';

  import "ol/ol.css";
  import { fromLonLat } from 'ol/proj.js';
  import Map from 'ol/Map.js';
  import OSM from 'ol/source/OSM.js';
  import VectorSource from 'ol/source/Vector.js';
  import View from 'ol/View.js';
  import Feature from 'ol/Feature.js';
  import Point from 'ol/geom/Point.js';
  import TileLayer from 'ol/layer/Tile.js';
  import WebGLPointsLayer from 'ol/layer/WebGLPoints.js';
  import VectorLayer from 'ol/layer/Vector';

  export let map = undefined;
  // export let selectedFeature = null;

  onMount(() => {
    // create the vector source that contains the vehicles to render
    let vectorSource = new VectorSource({
      features: [
        new Feature({
          type: 'geoMarker',
          geometry: new Point(fromLonLat([18.071327209472656, 59.34563446044922])),
        }),
        new Feature({
          type: 'geoMarker',
          geometry: new Point(0,0),
        }),
      ],
    });

    // create the vehicle rendering layer
    let vehicleLayer = new WebGLPointsLayer({
      source: vectorSource,
      // style: {

      //   symbol: {
      //     symbolType: 'circle',
      //     size: 15,
      //     color: 'rgb(255, 0, 0)',
      //     opacity: 0.7,
      //   },
      // },
      style: {
        'circle-radius': [
          'interpolate',
          ['exponential', 2],
          ['zoom'],
          5, // min zoom level
          2, // min size
          18, // max zoom level
          16, // max size
        ],
        'circle-fill-color': 'rgb(255, 0, 0)',
        // 'circle-displacement': [0, 0],
        'circle-opacity': 1.0,
      },
      disableHitDetection: false,
    });

    // let vehicleLayer = new VectorLayer({
    //   source: vectorSource,
    // });

    // create the map
    map = new Map({
      target: 'map-container',
      layers: [
        new TileLayer({
          source: new OSM(),
        }),
        vehicleLayer,
      ],
      view: new View({
        center: fromLonLat([18.071327209472656, 59.34563446044922]),
        zoom: 16,
      }),
    });


    // put vehicle features on the layer
    // vectorSource.addFeatures([
    //   new Feature({
    //     type: 'geoMarker',
    //     geometry: new Point(fromLonLat([18.071327209472656, 59.34563446044922])),
    //   })
    // ]);

    // // detect clicked vehicles
    // // maybe use Selecte({}) interaction instead
    // map.on('TODO: click', function (ev) {
    //   if (selectedFeature !== null) {
    //     selectedFeature.set('selected', false);
    //     selectedFeature = null;
    //   }

    //   map.forEachFeatureAtPixel(ev.pixel, function (feature) {
    //     feature.set('selected', true);
    //     selectedFeature = feature;
    //     return true;
    //   });
    // });

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
