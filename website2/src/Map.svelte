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
  import VectorLayer from 'ol/layer/Vector';
  import { getVectorContext } from 'ol/render.js';
  import { Circle, Fill, Icon, Stroke, Style } from 'ol/style.js';

  export let map = undefined;
  // export let selectedFeature = null;

  let frameCounter = 0;
  let renderVehicles = function(event) {
    let ctx = getVectorContext(event)
    ctx.setStyle(new Style({
      image: new Circle({
        radius: 7,
        fill: new Fill({color: 'black'}),
        stroke: new Stroke({
          color: 'white',
          width: 2,
        }),
      }),
    }));
    ctx.drawGeometry(new Point(fromLonLat([18.071327209472656+frameCounter*0.00001, 59.34563446044922])));

    frameCounter++;
    map.render();
  }

  onMount(() => {
    // create the vector source that contains the vehicles to render
    let vectorSource = new VectorSource();

    let vehicleLayer = new VectorLayer({
      source: vectorSource,
    });

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

    vehicleLayer.on('postrender', renderVehicles)


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
