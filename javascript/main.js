function renderItems(items, processType, elementId, processFunction) {
    let placeholder = "<div>";
    let itemsMeta = []; //we place the items title in order to add listener later on
    for(i = 0; i < items.length; i++) {
        let title = items[i]["title"];
        let placeholderId = processType + "-" + title.replace(" ", "-");
        placeholder += "<div>" + title + "<button " + 'id="' + placeholderId + '">' +
            processType + '</button>' + "</div>";
        itemsMeta.push({"id": placeholderId, "title":title});
        placeholder += "</div>";
        document.getElementById(elementId).innerHTML = placeholder;

        for (i = 0; i < itemsMeta.length; i++) {
            document.getElementById(itemsMeta[i]["id"]).addEventListener("click", processFunction);
        }
    }
}

function apiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener('readystatechange', function () {
        if (this.readyState == this.DONE) {
            renderItems(JSON.parse(this.responseText)["pending_items"], "edit", "pendingItems", editItem);
            renderItems(JSON.parse(this.responseText)["done_items"], "delete", "doneItems", deleteItem);
        }
    });
    xhr.open(method, url);
    xhr.setRequestHeader('content-type', 'application/json');
    xhr.setRequestHeader('user-token', 'token');
    return xhr
}

function editItem() {
    let title = this.id.replaceAll("-", " ").replace("edit ", "");
    let call = apiCall("/item/edit", "PUT");
    let json = {
        "title": title,
        "status": "done"
    };
    console.log("Editing Items: " + JSON.stringify(json));
    call.send(JSON.stringify(json));
}

function deleteItem() {
    let title = this.id.replaceAll("-", " ").replace("delete ", "");
    let call = apiCall("/item/delete", "POST");
    let json = {
        "title": title,
        "status": "done"
    };
    console.log("Deleting Item: " + JSON.stringify(json));
    call.send(JSON.stringify(json));
}

function getItems() {
    console.log("Getting Items");
    let call = apiCall("/item/get", 'GET');
    call.send();
}

getItems();

document.getElementById("create-button").addEventListener("click", createItem);

function createItem() {
    let title = document.getElementById("name");
    let call = apiCall("/item/create/" + title.value, 'POST');
    console.log("Creating Item: "+ title.value);
    call.send();
    document.getElementById("name").value = null;
}