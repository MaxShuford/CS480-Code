"use strict";

// $ function 
const $$ = selector => document.querySelector(selector);

let numOfWaypoints = 0;

const getRoutes = () => {

    console.log("Calulating Routes");

    const startBox = $$("[name=startCity]");
    const destinationBox = $$("[name=destinationCity]");


    const startCity = startBox.value;
    const destinationCity = destinationBox.value;
    
    console.log(startCity);
    console.log(destinationCity);


    let validStart = !(startCity.includes(", "));
    let validEnd = !(destinationCity.includes(", "));

    console.log(validStart);
    console.log(validEnd);
    //Check if the city input had valid format
    if(validStart || validEnd){
        alert('INVALID FORMAT');
    }

    if(numOfWaypoints > 0){
        const allWaypoints =  document.querySelectorAll(".waypoints");

        for(let i = 0; i < allWaypoints.length; i++){

            //get current waypoint value
            let wpCity = allWaypoints[i].value;
            console.log(wpCity + "Waypoint " + i);

            if(!(wpCity.includes(", "))){
                alert("Invalid Format");
                break;
            }
        }
    }


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
    console.log('Success:', data);
    const image = JSON.parse(data).image
    $("img").src = "data:image/png;base64," + image;
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
    $$("[name=startCity]").focus();  
});