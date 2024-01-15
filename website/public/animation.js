{ // we use JS scoping, to make sure nothing leaks into global scope
	const smoothZoomLevel = 10;

	// smooth animation only if zoomed in
	let animateSmooth = map.getZoom() > smoothZoomLevel;
	let mapIsMoving = false; // animation is paused during map movements

	map.on("movestart", function() {
		mapIsMoving = true;
	})
	map.on("zoomstart", function() {
		mapIsMoving = true;
	})
	map.on("zoomend", function() {
		refreshVehiclesOnScreen()
		mapIsMoving = false;
		animateSmooth = this.getZoom() > smoothZoomLevel;
		canvasLayer.animate();
	})
	// update vehiclesOnScreen when the map moves
	map.on('moveend', function(e) {
		refreshVehiclesOnScreen()
		mapIsMoving = false;
		canvasLayer.animate();
	});

	// vehicle icons
	const busIcon = new Image();
	busIcon.src = "./images/bus_icon.png";
	const trainIcon = new Image();
	trainIcon.src = "./images/train_icon.png";
	const tramIcon = new Image();
	tramIcon.src = "./images/tram_icon.png";
	const metroIcon = new Image();
	metroIcon.src = "./images/metro_icon.png";
	const ferryIcon = new Image();
	ferryIcon.src = "./images/ferry_icon.png";
	const taxiIcon = new Image();
	taxiIcon.src = "./images/taxi_icon.png";
	const otherIcon = new Image();
	otherIcon.src = "./images/other_icon.png";
	const iconsByColor = {
		"#FF7600": trainIcon,
		"#D61355": metroIcon,
		"#0078FF": busIcon,
		"#2BA714": tramIcon,
		"#0D1282": ferryIcon,
		"#FBCB0A": taxiIcon,
		"#8B8B8B": otherIcon,
	}

	// our custom canvasLayer, used to render vehicles
	const canvasLayer = new L.CustomLayer({
		container: document.createElement("canvas"),
		maxZoom: 19,
	});
	canvasLayer.animate = function() {
		// frame function is called every frame
		let layer = this;
		let frameCounter = 0;
		function frame(timestamp) {
			if (mapIsMoving) {
				return // pause animation when scrolling
			}
			if (animateSmooth || frameCounter % 60 == 0) {
				let canvas = layer.getContainer();
				let ctx = canvas.getContext("2d");
				ctx.clearRect(0, 0, canvas.width, canvas.height);

				// draw vehicles
				let pointRadius = zoomToPointRadius(layer._map.getZoom());
				vehiclesOnScreen.forEach(function(vehicle, _, _) {
					let point;
					if (vehicle.animateUntil < timestamp) {
						vehicle.animatedLatlng = null;
						point = layer._map.latLngToContainerPoint(vehicle.realLatlng);
						vehicle.containerPoint = point;
					} else {
						let animationDuration = vehicle.animateUntil - vehicle.animationStart;
						let remainingTime = vehicle.animateUntil - timestamp;
						let percentDone = (animationDuration - remainingTime) / (animationDuration + 1);
						let startPoint = layer._map.latLngToContainerPoint(vehicle.animationStartLatlng);
						let endPoint = layer._map.latLngToContainerPoint(vehicle.realLatlng);
						point = endPoint.multiplyBy(percentDone).add(startPoint.multiplyBy(1 - percentDone));
						vehicle.containerPoint = point;
						if (animateSmooth) {
							vehicle.animatedLatlng = layer._map.containerPointToLatLng(point);
						} else {
							vehicle.animatedLatlng = null;
						}
					}

					// draw
					if (pointRadius < 7) {
						ctx.beginPath();
						ctx.fillStyle = vehicle.color;
						ctx.arc(point.x, point.y, pointRadius, 0, 2*Math.PI);
						ctx.fill();
					} else {
						ctx.drawImage(iconsByColor[vehicle.color], point.x-pointRadius, point.y-pointRadius, 2*pointRadius, 2*pointRadius);
					}
				});
				// highlight selected vehicle
				if (selectedVehicle && vehiclesOnScreen.has(selectedVehicle.id)) {
					// draw the vehicle again on top
					if (pointRadius < 7) {
						ctx.beginPath();
						ctx.fillStyle = selectedVehicle.color;
						ctx.arc(selectedVehicle.containerPoint.x, selectedVehicle.containerPoint.y, pointRadius, 0, 2*Math.PI);
						ctx.fill();
					} else {
						ctx.drawImage(iconsByColor[selectedVehicle.color], selectedVehicle.containerPoint.x-pointRadius, selectedVehicle.containerPoint.y-pointRadius, 2*pointRadius, 2*pointRadius);
					}
					// draw highlight circle
					ctx.beginPath();
					ctx.strokeStyle = "red";
					ctx.lineWidth = 3
					ctx.arc(selectedVehicle.containerPoint.x, selectedVehicle.containerPoint.y, pointRadius+1, 0, 2*Math.PI);
					ctx.stroke();
					// draw textbox
					let textMetrics = ctx.measureText(selectedVehicle.displayText);
					let textHeight = textMetrics.actualBoundingBoxAscent + textMetrics.actualBoundingBoxDescent;
					let textWidth = textMetrics.actualBoundingBoxLeft + textMetrics.actualBoundingBoxRight;
					let textLeft = selectedVehicle.containerPoint.x + 3;
					let textBottom = selectedVehicle.containerPoint.y - pointRadius - 7 - textMetrics.actualBoundingBoxDescent;
					let textTop = textBottom - textMetrics.actualBoundingBoxAscent;
					ctx.beginPath();
					ctx.fillStyle = "white";
					ctx.rect(textLeft-2, textTop-2, textWidth+5, textHeight+5);
					ctx.fill();
					ctx.beginPath();
					ctx.strokeStyle = "black";
					ctx.lineWidth = 2
					ctx.rect(textLeft-2, textTop-2, textWidth+5, textHeight+5);
					ctx.stroke();
					ctx.fillStyle = "black";
					ctx.fillText(selectedVehicle.displayText, textLeft, textBottom);
				}
				// draw user location
				if (userPosition) {
					let point = layer._map.latLngToContainerPoint(userPosition.latlng);
					ctx.fillStyle = "#24b6ff";
					ctx.beginPath();
					ctx.arc(point.x, point.y, 6, 0, 2*Math.PI);
					ctx.fill();
					ctx.beginPath();
					ctx.globalAlpha = 0.3;
					ctx.arc(point.x, point.y, metersToPixels(userPosition.accuracy, map), 0, 2*Math.PI);
					ctx.fill();
					ctx.globalAlpha = 1.0;
				}
			}
			frameCounter++;
			window.requestAnimationFrame(frame);
		}

		// reset bounds and start the animation loop
		let { ctx } = layer.setFullLayerBounds();
		ctx.font = "20px arial"
		frame(performance.now());
	}
	// event handlers for our custom canvasLayer
	canvasLayer.on("layer-mounted", function() {
		this._reset();
		this.animate();
	});

	canvasLayer.addTo(map);
}
