"use strict";

const loadFavorites = () => {

    const postData = {
        uuid: parseInt(localStorage.getItem("userID"))
    };

    fetch('/retrieveFavorites', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(postData)
    })
    .then(response => response.json())
    .then(data => {

        console.log("Favorites loaded:", data);

        const list = document.querySelector("#favorites ul");

        // destroy accordion so DOM updates correctly
        $("#accordion").accordion("destroy");

        list.innerHTML = "";

        const favorites = data.favorites;

        for (let i = 0; i < favorites.length; i++) {
            createFavorite(favorites[i].name, favorites[i].route_id);
        }

        // rebuild accordion
        $("#accordion").accordion({
            collapsible: true,
            active: false
        });

    })
    .catch(error => console.error(error));
};

const getFavorite = (Event) => {

    const postData = {
        uuid: parseInt(localStorage.getItem("userID")),
        route_id: parseInt(Event.target.id)
    };

    fetch('/retrieveFavorite', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(postData)
    })
    .then(response => response.json())
    .then(data => {

        console.log('Success:', data);

        const routeLength = data.wp.length;
        let waypointOBJ = [];
        let completed = 0;

        for(let i = 0; i < routeLength; i++)
        {
            const wpcity = data.wp[i].name;
            const wpSplit = wpcity.split(", ");

            const postData = {city:wpSplit[0], state:wpSplit[1]};

            fetch('/locationData', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(postData)
            })
            .then(response => response.json())
            .then(locData => {

                waypointOBJ[i] = locData;
                waypointOBJ[i].id = i;
                waypointOBJ[i].name = wpcity;

                completed++;

                if(completed === routeLength)
                {
                    localStorage.setItem("start", waypointOBJ[0].name);
                    localStorage.setItem("destination", waypointOBJ[routeLength-1].name);

                    fetch('/directions', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify(waypointOBJ)
                    })
                    .then(response => response.json())
                    .then(data => {

                        localStorage.setItem("routes", JSON.stringify(data.routes));
                        window.location.href = "/html/Route.html";

                    });
                }
            });
        }

    });
};

const removeFavorite = (ActionEvent) => {

    const routeDiv = ActionEvent.target.closest(".favRoute");
    const routeID = parseInt(routeDiv.id);

    const postData = {
        uuid: parseInt(localStorage.getItem("userID")),
        route_id: routeID
    };

    fetch('/deleteFavorite', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(postData),
    })
    .then(response => response.json())
    .then(data => {

        console.log('Success:', data);

        // remove item from UI immediately
        routeDiv.parentElement.remove();

    })
    .catch((error) => {
        console.error('Error:', error);
    });
};

const createFavorite = (name, routeID) => {

    console.log("Creating Favorite to list");

    const listOfFavorite = document.querySelector("#favorites ul");

    const listItem = document.createElement("li");

    const fullDiv = document.createElement("div");
    fullDiv.classList.add("favRoute");
    fullDiv.id = routeID;

    const firstHalfDiv = document.createElement("div");
    firstHalfDiv.classList.add("FavPartOne");

    const routeParts = name.split("-");

    const startCity = document.createElement("span");
    startCity.textContent = routeParts[0];
    firstHalfDiv.appendChild(startCity);

    const arrowImg = document.createElement("img");
    arrowImg.classList.add("arrow");
    arrowImg.src = "../css/Images/ArrowIcon.png";
    arrowImg.alt = "RightArrow";
    firstHalfDiv.appendChild(arrowImg);

    const endCity = document.createElement("span");
    endCity.textContent = routeParts[1];
    firstHalfDiv.appendChild(endCity);

    const secondHalfDiv = document.createElement("div");
    secondHalfDiv.classList.add("FavPartTwo");

    const trashImg = document.createElement("img");
    trashImg.classList.add("trash");
    trashImg.src = "../css/Images/TrashIcon.png";
    trashImg.alt = "Trash";
    trashImg.addEventListener("click", removeFavorite);
    secondHalfDiv.appendChild(trashImg);

    const goButton = document.createElement("input");
    goButton.type = "button";
    goButton.value = "GO";
    goButton.classList.add("buttons");
    goButton.id = routeID;
    goButton.addEventListener("click", getFavorite);
    secondHalfDiv.appendChild(goButton);

    fullDiv.appendChild(firstHalfDiv);
    fullDiv.appendChild(secondHalfDiv);

    listItem.appendChild(fullDiv);
    listOfFavorite.appendChild(listItem);
};

document.addEventListener("DOMContentLoaded", () => {
    loadFavorites();
});
