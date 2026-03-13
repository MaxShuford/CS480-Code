"USE STRICT";

const $ = selector => document.querySelector(selector);
let routes = [];
document.addEventListener("DOMContentLoaded", () => {
    $("button").addEventListener("click", event => {
        favorite();
    });
    routes = JSON.parse(localStorage.getItem("routes"));
    showImage();
    showDirections();

    $(".backButton").addEventListener("click", backToRoutes);
});

function showImage()
{
    console.log(routes);
    const postData = [];
    for(let i = 0; i < routes.length;i++)
    {
        postData[i]= {route:{route_id:i, wp:routes[i].waypoints}, geometry:routes[i].geometry};
    }
    console.log("post", postData);
    console.log(JSON.stringify(postData))
    fetch('/mapWithRoutes', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    console.log('Success:', data);
    $("#theMap").src = "data:image/png;base64," + data.image;
    })
    .catch((error) => {
    console.error('Error:', error);
    });
}

function showDirections()
{
    for (let i = 0; i < routes.directions.length; i++){
        const newLi = document.createElement("li");
        newLi.textContent = routes.directions[i];
        $("aside ul").appendChild(newLi);
    }
}

function favorite()
{
    const postData = { uid: localStorage.getItem('userID'), route: localStorage.getItem('route') };

    fetch('/addToFavorite', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    console.log('Success:', data);
    })
    .catch((error) => {
    console.error('Error:', error);
    });
}

//Back to route button PROTOTYPE
const backToRoutes = () => {

    //What data should be brought back to


    location.href = "Route.html";
}