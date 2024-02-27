function confirmEmployeeLogout() {
    Swal.fire({
        title: "Warning!",
        text: "Are you sure you want to log out as an empoyee?",
        icon: "warning",
        confirmButtonText: "Log Me Out",
        confirmButtonColor: "#d33",
    }).then((result) => {
        if (result.isConfirmed) {
            logoutEmployee()
        }
    })
}

function confirmAdminLogout() {
    Swal.fire({
        title: "Warning!",
        text: "Are you sure you want to log out as an admin?",
        icon: "warning",
        confirmButtonText: "Log Me Out",
        confirmButtonColor: "#d33",
    }).then((result) => {
        if (result.isConfirmed) {
            logoutAdmin()
        }
    })
}
