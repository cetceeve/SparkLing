function initiateLeaflet() {
    let map = L.map('map', { renderer: L.canvas() }).setView([59.32249871, 18.070166386], 12);
    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);
    // L.tileLayer('https://tile.thunderforest.com/pioneer/{z}/{x}/{y}.png?apikey=131eec8b12b3471cb55357e19c1c1b62', {
    //     maxZoom: 19,
    // }).addTo(map);
    return map;
}

// define stream source
const evtSource = new EventSource(window.location.origin + "/realtime");
// marker storage
const vehicleHM = new Map();

// start
const map = initiateLeaflet();

// main event loop
evtSource.onmessage = (event) => {
    let data = JSON.parse(event.data);
   
    if (vehicleHM.has(data.vehicle_id)) {
        vehicleHM.get(data.vehicle_id).updateData(data)
    } else {
        vehicleHM.set(data.vehicle_id, new Vehicle(data));
        vehicleHM.get(data.vehicle_id).addTo(map);
    }
};







