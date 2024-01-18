// data stores for reactive ui
document.addEventListener('alpine:init', () => {
    Alpine.store('selectedVehicle', {
        displayText: "transitmap.io",
        stops: [],
        delay: [],
        firstPredictedSequence: 1000,
        agencyName: "",
        
        update(vehicle) {
            this.displayText = vehicle.getDisplayText();
            this.stops = vehicle.stops ? vehicle.stops : [];
            this.delay = vehicle.delay ? vehicle.delay : [];
            this.firstPredictedSequence = vehicle.first_predicted_sequence ? vehicle.first_predicted_sequence : 1000;
            this.agencyName = vehicle.agency_name ? vehicle.agency_name : "";
        }
    })
})
