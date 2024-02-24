function confirmLogout() {
    Swal.fire({
        title: "Warning!",
        text: "Are you sure you want to log out?",
        icon: "warning",
        confirmButtonText: "Log Me Out",
        confirmButtonColor: "#d33",
    }).then((result) => {
        if (result.isConfirmed) {
            window.location.href = "./login.html";
        }
    })
}
