// data stores for reactive ui
document.addEventListener('alpine:init', () => {
    Alpine.store('selectedVehicle', {
        displayText: "transitmap.io",
        m: {},
        delay: [],
        firstPredictedSequence: 1000,
        
        update(vehicle) {
            this.displayText = vehicle.displayText;
            this.m = vehicle.metadata ? vehicle.metadata : {};
            this.delay = vehicle.delay ? vehicle.delay : [];
            this.firstPredictedSequence = vehicle.firstPredictedSequence ? vehicle.firstPredictedSequence : 1000;
        }
    })
})
