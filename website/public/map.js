let map = L.map('map').setView([59.32249871, 18.070166386], 12);
L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
    maxZoom: 19,
    attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
}).addTo(map);
// L.tileLayer('https://tile.thunderforest.com/pioneer/{z}/{x}/{y}.png?apikey=131eec8b12b3471cb55357e19c1c1b62', {
//     maxZoom: 19,
// }).addTo(map);
let marker = L.marker([59.35008532389274, 18.070244623562843]).addTo(map);

const evtSource = new EventSource(window.location.origin + "/realtime");

evtSource.onmessage = (event) => {
    console.log(event)
};
  
