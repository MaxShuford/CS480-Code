"use strict";

// $ function 
const $$ = selector => document.querySelector(selector);

let numOfWaypoints= 0;

const getRoutes = async () => {

    const allWaypoints = document.querySelectorAll(".cityBox");
    console.log("Calculating Routes");

    let waypointOBJ = [];

    const requests = [];

    for (let i = 0; i < allWaypoints.length; i++) {

        let wpCity = allWaypoints[i].value;

        if (!wpCity.includes(", ")) {
            alert("Invalid Format");
            return;
        }

        const wpSplit = wpCity.split(", ");
        const postData = { city: wpSplit[0], state: wpSplit[1] };

        requests.push(
            fetch('/locationData', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(postData)
            })
            .then(response => response.json())
            .then(data => {

                data.id = i;
                data.name = wpCity;

                waypointOBJ[i] = data;
            })
        );
    }
    // ASYNC RUST FOR THE WIN
    await Promise.all(requests);

    console.log("All waypoints resolved:", waypointOBJ);

    localStorage.setItem("start", waypointOBJ[0].name);
    localStorage.setItem("destination", waypointOBJ[waypointOBJ.length - 1].name);

    fetch('/directions', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(waypointOBJ)
    })
    .then(response => response.json())
    .then(data => {
        console.log("Routes received:", data);
        localStorage.setItem("routes", JSON.stringify(data.routes));
        window.location.href = "/html/Route.html";
    })
    .catch(error => {
        console.error(error);
    });
}


const createWaypoint = () => {

    console.log("Creating Waypoint");

    if(numOfWaypoints >= 5){
        alert("Can't have more than 5 waypoints!");
        return;
    }
    
    //Increase increment
    numOfWaypoints++;

    const endBox = $$("#endTextbox");

    //Create a new div element
    const newDiv = document.createElement("div");
    newDiv.classList.add("WaypointDiv_" + numOfWaypoints);
    const newInput = document.createElement("input");

    //Create a new text input element
    newInput.classList.add("cityBox");
    newInput.classList.add("waypoints");
    newInput.placeholder = "Waypoint " + numOfWaypoints;

    //Create a new img element
    const newImg = document.createElement("img");
    newImg.classList.add("arrow");
    newImg.src = "../css/Images/ArrowIcon.png";
    newImg.alt = "DownArrow";

    //Make TextInput and img as child elements of div
    newDiv.appendChild(newInput);
    newDiv.appendChild(newImg);

    //Place Div above last box
    endBox.insertAdjacentElement('beforebegin', newDiv);

    if(numOfWaypoints == 5){
        const addButton = $$("#wayAdd");
        addButton.remove();
    }

}

const showUserLocation = () => {
    navigator.geolocation.getCurrentPosition(function(position) {
    const lat = position.coords.latitude;
    const lng = position.coords.longitude;
    console.log("Latitude:", lat, "Longitude:", lng);

    const postData = {latitude: lat, longitude: lng};
    fetch('/mapWithUserLoc', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    $$("#theMap").src = "data:image/png;base64,"+data.image;
    })
    .catch((error) => {
    console.error('Error:', error);
    });
  });
}


//Runs accordian function
$(document).ready(function(){
        
        //creates an accordian at id=accordian
        $("#accordion").accordion({ 
                event: "click",         //when click, it expense the accordian
                heightStyle: "content", //the height of the accordian
                collapsible: true,      //make all of them collapsible
                active:false,           //make sure when loaded, all links are closed
        });
});

//DomContentLoaded
document.addEventListener("DOMContentLoaded", () =>{
    showUserLocation();
    $$("#GetRoute").addEventListener("click", getRoutes);
    $$("#wayAdd").addEventListener("click", createWaypoint);
    $$("#logout").addEventListener("click", logout);
    $$("[name=startCity]").focus();  
});