"USE STRICT";

const $ = selector => document.querySelector(selector);
let routes = [];
document.addEventListener("DOMContentLoaded", () => {
    $("#favoriteButton").addEventListener("click", event => {
        favorite();
    });
    $("#logout").addEventListener("click", logout);
    routes = JSON.parse(localStorage.getItem("routes"));
    showImage();
    showDirections();

});

function showImage()
{
    console.log(routes);
    
    let postData = [{route:{route_id:0, wp:routes.waypoints}, geometry:routes.geometry}];
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

        if(i % 2 == 0)
            newLi.classList.add("evenItem");
        else
            newLi.classList.add("oddItem");

        $("aside ul").appendChild(newLi);
    }
}

function favorite()
{
    console.log("favorite button clicked");
    const uuid = parseInt(localStorage.getItem("userID"));

    const storedRoutes = JSON.parse(localStorage.getItem("routes"));
    const route = storedRoutes.waypoints;

    if (!uuid || !route) {
        console.error("Missing userID or route");
        return;
    }

    const postData = {
        uuid: uuid,
        wp: route
    };

    fetch('/addToFavorite', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(postData)
    })
    .then(response => response.json())
    .then(data => {
        console.log("Favorite saved:", data);
    })
    .catch(error => {
        console.error("Error:", error);
    });
}