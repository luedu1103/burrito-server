const defaultLat = -12.057478;  // -12.057478, -77.083282
const defaultLon = -77.083282;
const initialZoom = 17; // Initial zoom level

// Define the bounds of the allowed area
const southWest = L.latLng(-12.064890, -77.090576); //-12.064890, -77.090576 
const northEast = L.latLng(-12.048270, -77.077165); // -12.050621, -77.077165
const bounds = L.latLngBounds(southWest, northEast);

const map = L.map('map', {
    center: [defaultLat, defaultLon],
    zoom: initialZoom,
    zoomControl: true,  // Enable zoom control buttons
    // maxBounds: bounds,  // Set the boundaries
    // maxBoundsViscosity: 1.0,  // Strictly limit the view to the boundaries
    // minZoom: 16,  // Minimum zoom level allowed
    // maxZoom: 18,  // Maximum zoom level allowed
    scrollWheelZoom: true, // Enable scroll wheel zoom
    doubleClickZoom: true, // Enable double click zoom
    boxZoom: true, // Enable box zoom
    dragging: true // Enable dragging
});

L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
}).addTo(map);

const customIcon = L.icon({
    iconUrl: 'static/img/burro.png',
    iconSize: [25, 30],
    iconAnchor: [12, 41],
    popupAnchor: [1, -34],
    shadowUrl: 'https://unpkg.com/leaflet@1.7.1/dist/images/marker-shadow.png',
    shadowSize: [41, 41]
});

const paraderoIcon = L.icon({
    iconUrl: 'static/img/paradero.png',
    iconSize: [25, 30],
    iconAnchor: [12, 41],
    popupAnchor: [1, -34],
    shadowUrl: 'https://unpkg.com/leaflet@1.7.1/dist/images/marker-shadow.png',
    shadowSize: [41, 41]
});

// Add markers at specific points
const markerPoints = [
    { lat: -12.053741, lon: -77.085337, label: 'Paradero Sistemas' },
    { lat: -12.054815, lon: -77.086370, label: 'Paradero Geología' }, // -12.054815, -77.086370
    { lat: -12.056256, lon: -77.085075, label: 'Paradero Biblioteca' }, // -12.056256, -77.085075
    { lat: -12.060182, lon: -77.084389, label: 'Paradero Gimnasio' }, // -12.060182, -77.084389
    { lat: -12.060365, lon: -77.080690, label: 'Paradero Industrial' }, // -12.060365, -77.080690
    { lat: -12.059567, lon: -77.079650, label: 'Puerta 2' },// -12.059567, -77.079650
    { lat: -12.057668, lon: -77.080122, label: 'Puerta 3' },// -12.057668, -77.080122
    { lat: -12.055558, lon: -77.082043, label: 'Paradero Clínica' },// -12.055558, -77.082043
    { lat: -12.054160, lon: -77.084444, label: 'Paradero Educación Física' },// -12.054160, -77.084444
];

markerPoints.forEach(point => {
    const marker = L.marker([point.lat, point.lon], { icon: paraderoIcon }).addTo(map)
        .bindPopup(point.label);
});

const markerActualPosition =  L.marker([defaultLat, defaultLon], { icon: customIcon }).addTo(map)
    .setPopupContent(`Coordinates: ${defaultLat}, ${defaultLon}`)
    .openPopup();

async function fetchPosition(count = 1) {
    try {
        const response = await fetch(`https://burrito-server.shuttleapp.rs/get-position/${count}`);
        if (!response.ok) throw new Error('Network response was not ok');
        const data = await response.json();

        if (data.positions && data.positions.length > 0) {
            // Return all positions or the latest one
            const positions = data.positions.map(pos => ({
                lat: pos.lt,
                lon: pos.lg,
                humidity: pos.hum,
                temperature: pos.tmp,
                status: interpretStatus(pos.sts),
            }));
            return positions;
        } else {
            throw new Error('Invalid data');
        }
    } catch (error) {
        console.error('Error fetching position:', error);
        return [{
            lat: defaultLat,
            lon: defaultLon,
            humidity: 0,
            temperature: 0,
            status: 'Unknown',
        }];
    }
}

// Función para interpretar el estado basado en el código numérico
function interpretStatus(statusCode) {
    switch (statusCode) {
        case 0:
            return 'En ruta';
        case 1:
            return 'Fuera de servicio';
        case 2:
            return 'En descanso';
        case 3:
            return 'Accidente';
        case 4:
            return 'Error';
        default:
            return 'Desconocido';
    }
}

async function updatePosition() {
    try {
        const positions = await fetchPosition(1); // Obtener la última posición

        if (positions && positions.length > 0) {
            const latestPosition = positions[0]; // Obtener la última posición del array

            const lat = latestPosition.lat;
            const lon = latestPosition.lon;

            // Actualizar la posición del marcador y el contenido del popup
            markerActualPosition.setIcon(customIcon)
                .setLatLng([lat, lon])
                .setPopupContent(`
                    Coordinates: ${lat}, ${lon}<br>
                `)
                .openPopup();

             // Actualizar los datos de velocidad, humedad, temperatura y estado
             document.getElementById('humidity').textContent = latestPosition.humidity.toFixed(2);
             document.getElementById('temperature').textContent = latestPosition.temperature.toFixed(2);
             document.getElementById('status').textContent = latestPosition.status;

            // Centrar el mapa en la posición actual sin cambiar el nivel de zoom
            map.setView([lat, lon], map.getZoom(), { animate: true, duration: 1 });
        }
    } catch (error) {
        console.error('Error updating position:', error);
    }
}

// Update position periodically
setInterval(updatePosition, 1000);

// Fetch and add GeoJSON data to the map
async function addGeoJSON() {
    try {
        const response = await fetch('static/img/UNMSM.json'); // Adjust the path to your GeoJSON file
        if (!response.ok) throw new Error('Network response was not ok');
        const geojsonData = await response.json();

        // Define a style function to style each feature differently
        function style(feature) {
            return {
                color: getColor(feature.properties.type), // Get color based on the feature type
                weight: 2,
                opacity: 1,
                fillOpacity: 0.7
            };
        }

        // Define a function to get a color based on a feature property
        function getColor(type) {
            switch (type) {
                case 'type1': return '#ff0000'; // Red for type1
                case 'type2': return '#00ff00'; // Green for type2
                case 'type3': return '#0000ff'; // Blue for type3
                default: return '#ff7800'; // Default color
            }
        }

        // Add GeoJSON data to the map with custom styles
        L.geoJSON(geojsonData, {
            style: style
        }).addTo(map);
    } catch (error) {
        console.error('Error fetching GeoJSON data:', error);
    }
}

// Call the function to add GeoJSON data
addGeoJSON();

// 
// VELOCITY
// 

async function fetchVelocity() {
    try {
        const response = await fetch('https://burrito-server.shuttleapp.rs/get-velocity');
        if (!response.ok) throw new Error('Network response was not ok');
        const data = await response.json();
        const velocityElement = document.getElementById('velocity');
        if (velocityElement) {
            velocityElement.textContent = data.velocity.toFixed(2); // Mostrar la velocidad redondeada a dos decimales
        }
    } catch (error) {
        console.error('Error fetching velocity:', error);
    }
}

// Llama a fetchVelocity() para obtener y mostrar la velocidad inicialmente
fetchVelocity();

// Actualiza la velocidad cada cierto tiempo
setInterval(fetchVelocity, 5000);

// Fetch and add GeoJSON data to the map
async function addGeoJSONLines() {
    try {
        const response = await fetch('static/img/ruta.json'); // Ajusta la ruta a tu archivo GeoJSON
        if (!response.ok) throw new Error('Network response was not ok');
        const geojsonData = await response.json();

        // Define una función para estilizar cada feature
        function style(feature) {
            return {
                color: '#3388ff', // Color de las líneas
                weight: 3, // Grosor de las líneas
                opacity: 0.7 // Opacidad de las líneas
            };
        }

        // Agrega el GeoJSON al mapa con estilos personalizados
        L.geoJSON(geojsonData, {
            style: style
        }).addTo(map);
    } catch (error) {
        console.error('Error fetching or parsing GeoJSON:', error);
    }
}

// Llama a la función para agregar el GeoJSON al mapa
addGeoJSONLines();