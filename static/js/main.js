let socket = new WebSocket("wss://chat-ws.raince.ru");
let chat = document.getElementById("chat");


socket.onopen = function (e) {
    console.log("Соединение установлено!")
};


socket.onmessage = function (event) {
    console.log(`[message] Данные получены с сервера: ${event.data}`);

    if (event.data.startsWith("new_message: ")) {
        let data = event.data.replace("new_message: ", "");
        let data_list = data.split(";");
        let nickname = data_list[0];
        let msg = data_list[1];
        
        if (nickname == get_nickname()) {
            chat.innerHTML += `
            <li class="message-block my-message">
                <div>
                    <p class="nickname">
                        Your
                    </p>

                    <p class="message">
                        ${msg}
                    </p>
                </div>
            </li>`
        } else {
            chat.innerHTML += `
            <li class="message-block">
                <div>
                    <p class="nickname">
                        ${nickname}
                    </p>

                    <p class="message">
                        ${msg}
                    </p>
                </div>
            </li>`
        }
    }
};


socket.onclose = function (event) {
    if (event.wasClean) {
        console.log(`[close] Соединение закрыто чисто, код=${event.code} причина=${event.reason}`);
    } else {
        console.log('[close] Соединение прервано');
    }
};


socket.onerror = function (error) {
    console.log(`[error] ${error}`);
};


document.querySelector("#send_message").onclick = function(){
    let message = document.getElementById("input-message").value;

    socket.send(`send_message: ${get_nickname()};${message}`)

    document.getElementById("input-message").value = "";
}


function get_nickname() {
    let cookie = document.cookie;
    for (i of cookie.split("; ")) {
        if (i.startsWith("nickname=")) {
            return i.replace("nickname=", "")
        }
    }

    return "None"
}