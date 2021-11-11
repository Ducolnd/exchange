let protocol = "ws";
let conn = null;

$(document).ready(function() {
    connect();

    $("#transaction").submit(function(e) {
        let data = {
            size: parseInt(parseFloat($("#size").val())),
            price: parseInt(parseFloat($("#price").val())),
            sell: $("#sell:checked").val() === "on" ? true : false,
        }      
        data = JSON.stringify(data);
        e.preventDefault();

        if (protocol == "http") {
    
            $.ajax({
                url: 'http://127.0.0.1:8080/',
                type: 'post',
                data: data,
                headers: {"Content-Type": "application/json"},
                dataType: 'json',
                success: function (data) {
                    console.info(data);
                }
            });
        } else {
            conn.send(data);
        }
    })

    $("#ws").click(function() {
        connect();
        $(this).css("color", "blue");
        $("#http").css("color", "black");
        protocol = "ws"
    })

    $("#http").click(function() {
        disconnect();
        $(this).css("color", "blue");
        $("#ws").css("color", "black");
        protocol = "http"
    })
})

function connect() {
    disconnect();
    var url = "ws://127.0.0.1:8080/ws/"

    conn = new WebSocket(url);
    console.log("Connecting...");

    conn.onopen = function() {
        console.log("Connected");
    }
    conn.onmessage = function(e) {
        console.info("Received a message ", e.data);
    }
    conn.onclose = function() {
        console.warn("Disconnected from server")
        conn = null;
    }
}

function disconnect() {
    if (conn != null) {
        console.log('Disconnecting...');
        conn.close();
        conn = null;
    }
}