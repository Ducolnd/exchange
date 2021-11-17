'use strict';

let protocol = "ws";
let action = "sell";

$(document).ready(function () {
    connect();

    $("#submit_form").click(function () {
        let data = {
            size: parseInt(parseFloat($("#size").val())),
            price: parseInt(parseFloat($("#price").val())) * 10e9,
            sell: action === "sell" ? true : false,
            type: "Transaction",
        }
        data = JSON.stringify(data);

        if (protocol == "http") {
            $.ajax({
                url: 'http://127.0.0.1:8080/',
                type: 'post',
                data: data,
                headers: { "Content-Type": "application/json" },
                dataType: 'json',
                success: function (data) {
                    console.info(data);
                }
            });
        } else {
            conn.send(data);
        }
    });

    // Buy or sell select
    $("#select_sell").click(function () {
        $(this).attr("class", "btn btn-danger");
        $("#select_buy").attr("class", "btn btn-outline-success");

        action = "sell";
    })

    $("#select_buy").click(function () {
        $(this).attr("class", "btn btn-success");
        $("#select_sell").attr("class", "btn btn-outline-danger");

        action = "buy";
    })

    // Protocol select
    $("#ws").click(function () {
        connect();
        $(this).css("color", "blue");
        $("#http").css("color", "black");
        protocol = "ws"
    })

    $("#http").click(function () {
        disconnect();
        $(this).css("color", "blue");
        $("#ws").css("color", "black");
        protocol = "http"
    })
});

class Order extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            order: props.value,
        }
    }

    render() {
        return (
            <div className="row">
                <div className="col"><p>{this.state.order.size}</p></div>
                <div className="col price-col"><p>{this.state.order.price / 10e9}</p></div>
            </div>
        );
    }
}