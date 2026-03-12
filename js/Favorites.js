"use strict";

const getFavorites = (ActionEvent) => {

    const divObject = ActionEvent.target.closest(".favRoute");

    let idNum = divObject.id;

    console.log(idNum);

    const postData = {uid:localStorage.getItem("userID")};
    fetch('/retrieveFavorites', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    console.log('Success:', data);
    favorites = JSON.parse(data);
    })
    .catch((error) => {
    console.error('Error:', error);
    });

}


const removeFavorite = (ActionEvent) => {

    console.log(ActionEvent.value);
    const postData = {uid:localStorage.getItem("userID")};
    fetch('/delFavorite', {
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


/*

<li>
                                <div class="favRoute" id="1">

                                    <div class="FavPartOne">
                                        <span>City A</span>

                                        <img src="../css/Images/ArrowIcon.png" alt="LeftArrow" class="arrow">

                                        <span>City B</span>
                                    </div>

                                    <div class="FavPartTwo">
                                        <a><img src="../css/Images/TrashIcon.png" alt="Trash" class="trash"></a>

                                        <input type="button" value="GO" class="button" id="FavRoute">
                                    </div>
                                </div>
                            </li>



*/

const createFavorite = (name, routeID) => {

    console.log("Creating Favorite to list");

    let currentID = 0;

    //Get the UL
    const listOfFavorite = $$("#favorites").firstElementChild;

    //Create li element
    const listItem = document.createElement("li")

    //Create a new fulldiv element
    const fullDiv = document.createElement("div");
    fullDiv.classList.add("favRoute");
    fullDiv.id = currentID;

    //Create FirstHalf
    const firstHalfDiv = document.createElement("div");
    firstHalfDiv.classList.add("FavPartOne");

    //StartCity and set to child of firstHalf
    const startCity = document.createElement("span");
    startCity.textContent = name;
    firstHalfDiv.appendChild(startCity);

    //Create a new arrow img element and set as child of firstHalf
    const arrowImg = document.createElement("img");
    arrowImg.classList.add("arrow");
    arrowImg.src = "../css/Images/ArrowIcon.png";
    arrowImg.alt = "RightArrow";
    firstHalfDiv.appendChild(arrowImg);

    //EndCity and set to child of firstHalf
    const endCity = document.createElement("span");
    endCity.textContent = name;
    firstHalfDiv.appendChild(endCity);

    //Create SecondHalf
    const secondHalfDiv = document.createElement("div");
    secondHalfDiv.classList.add("FavPartTwo");

    //Create a new trash img element and set as child of secondHalf
    const trashImg = document.createElement("img");
    trashImg.classList.add("trash");
    trashImg.src = "../css/Images/TrashIcon.png";
    trashImg.alt = "Trash";
    secondHalfDiv.appendChild(trashImg);

    //Create GO button then set as child in secondHalf
    const goButton = document.createElement("input");
    goButton.type = "button";
    goButton.value = "GO";
    goButton.classList.add("button");
    fullDiv.id = "FavRoute";
    secondHalfDiv.appendChild(goButton)

    //Put halfs to full DIv
    fullDiv.appendChild(firstHalfDiv);
    fullDiv.appendChild(secondHalfDiv);

    //Put div to li
    listItem.appendChild(fullDiv);

    //Put li to Ul
    listOfFavorite.appendChild(listItem);
}

//DomContentLoaded
document.addEventListener("DOMContentLoaded", () =>{


    //GET FAVORITES FROM DB HERE
    const postData = {uid:localStorage.getItem("userID")};
    fetch('/retrieveFavorites', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    console.log('Success:', data);
    favorites = JSON.parse(data);
    })
    .catch((error) => {
    console.error('Error:', error);
    });
    //CREATE FAVORITES ITEMS HERE
    for(let i = 0; i < favorites.names.length; i++)
    {
        createFavorite(favorites.name[i], favorites.route_id[i]);
    }
    //Get all the add and trash buttons and give them their function.
    const allAddButtons = document.querySelectorAll("#FavRoute");
    const allTrashButtons = document.querySelectorAll(".trash")

    for(let i = 0; i < allAddButtons.length; i++){
        allAddButtons[i].addEventListener("click", getFavorites);
        allTrashButtons[i].addEventListener("click", removeFavorite);
    }
});