const Vehicle =  L.Circle.extend({
    initialize: function(data) {
        console.log(data);
        L.Circle.prototype.initialize.call(this, [data.latitude, data.longitude])
        L.setOptions(this, {
            radius: 10,
            color: data.trip_id ? "blue" : "red",
        });
        this.id = data.vehicle_id;
        // this.bearing = data.bearing;
        // this.speed = data.speed;
        this.lastUpdateTimestamp = performance.now();
    },
    updateData: async function(data) {
        console.log(data);
        let newTimestamp = performance.now();
        // L.Circle.prototype.slideCancel(); // TODO: is this a fix for the missing latlng???
        // L.Circle.prototype.slideTo.call(this,
        //     [data.latitude, data.longitude],
        //     { duration: newTimestamp - this.lastUpdateTimestamp});
        L.Circle.prototype.setStyle.call(this, { color: data.trip_id ? "blue" : "red"});
        // this.bearing = data.bearing;
        // this.speed = data.speed;
        await this.slideTo([data.latitude, data.longitude], newTimestamp, (newTimestamp - this.lastUpdateTimestamp) * 1.1);
        this.lastUpdateTimestamp = newTimestamp;
    },
    slideTo: async function(latlng, currTimestamp, duration) {
        this._slideToDuration = duration;
		this._slideToUntil    = currTimestamp + duration;
        this._slideFromLatLng = this.getLatLng();
		this._slideToLatLng   = latlng;
        
        if (this._slideFromLatLng.equals(this._slideToLatLng)) {
            return this;
        }

        if (!this._animationIsRunning) {
            this._animate(performance.now());
        }
    },
    _step: async function(execTimestamp) {
        if (!this._map) return true;

        let remaining = this._slideToUntil - execTimestamp;
        if (remaining < 0) {
			this.setLatLng(this._slideToLatLng);
            return true;
        }

        let startPoint = this._map.latLngToContainerPoint(this._slideFromLatLng);
		let endPoint   = this._map.latLngToContainerPoint(this._slideToLatLng);
		let percentDone = (this._slideToDuration - remaining) / this._slideToDuration;

		let currPoint = endPoint.multiplyBy(percentDone).add(startPoint.multiplyBy(1 - percentDone));
		let currLatLng = this._map.containerPointToLatLng(currPoint);
        this.setLatLng(currLatLng);
        return false
    },
    _animate: async function(timestamp) {
        this._animationIsRunning = true;
        
        isComplete = await this._step(timestamp);
        if (isComplete) {
            this._animationIsRunning = false;
        } else {
            L.Util.requestAnimFrame(this._animate, this);
        }
    },
})
