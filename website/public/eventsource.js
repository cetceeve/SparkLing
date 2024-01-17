// define stream source
const evtSource = new EventSource(window.location.origin + "/realtime");

// main event loop
evtSource.onmessage = (event) => {
    let data = JSON.parse(event.data);

    if (vehicleHM.has(data.id)) {
        let vehicle = vehicleHM.get(data.id);
        let onScreen = isOnScreen(vehicle, map);
        vehicle.updateData(data, onScreen);
        if (onScreen) {
            vehiclesOnScreen.set(vehicle.id, vehicle);
        } else if (vehiclesOnScreen.has(vehicle.id)) {
            vehiclesOnScreen.delete(vehicle.id);
        }
        // Trigger reactive update.
        if (vehicle.id === selectedVehicle?.id) {
            Alpine.store("selectedVehicle").update(vehicle);
        }
    } else {
        let vehicle = newVehicle(data);
        if (vehicle) {
            vehicleHM.set(vehicle.id, vehicle);
            if (isOnScreen(vehicle, map)) {
                vehiclesOnScreen.set(vehicle.id, vehicle);
            }
        }
    }
};
