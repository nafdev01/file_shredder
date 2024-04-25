const { invoke } = window.__TAURI__.core;

const employeeForm = document.querySelector('#employee-login-form');
const adminForm = document.querySelector('#admin-login-form');

employeeForm.addEventListener('submit', (event) => {
    event.preventDefault();

    const employeeUsername = document.querySelector('#employee-signin-username').value;
    const employeePassword = document.querySelector('#employee-signin-password').value;

    invoke('authenticate_employee', {
        username: employeeUsername,
        password: employeePassword,
    }).then(response => {
        let employee = response;
        loginEmployee(employee.employeeid, employee.username, employee.fullname);
        Swal.fire({
            title: `Welcome back ${employee.username}!`,
            html: `Please wait while we log you in <b></b>`,
            timer: 3000,
            didOpen: () => {
                Swal.showLoading();
                Swal.getPopup().querySelector("b");
            },
        }).then((result) => {
            if (result.dismiss === Swal.DismissReason.timer) {
                window.location.href = 'employee-dashboard.html';
            }
        });
    }
    ).catch(error => {
        // if the rror i lowercase is query retured no rows
        if (error.toString().toLowerCase().includes('query returned no rows')) {
            errorMessage = 'Invalid employee username or password';
        }
        else {
            errorMessage = error;
        }

        Swal.fire({
            title: 'Error!',
            text: errorMessage, // ensure error is a string
            icon: 'error',
            confirmButtonText: 'Ok'
        });
    });
});


adminForm.addEventListener('submit', (event) => {
    event.preventDefault();

    const adminUsername = document.querySelector('#admin-signin-username').value;
    const adminPassword = document.querySelector('#admin-signin-password').value;

    invoke('authenticate_admin', {
        username: adminUsername,
        password: adminPassword,
    }).then(response => {
        let admin = response;
        loginAdmin(admin.adminid, admin.username, admin.fullname);
        Swal.fire({
            title: `Welcome back ${admin.username}!`,
            html: `Please wait while we log you in <b></b>`,
            timer: 3000,
            didOpen: () => {
                Swal.showLoading();
                Swal.getPopup().querySelector("b");
            },
        }).then((result) => {
            if (result.dismiss === Swal.DismissReason.timer) {
                window.location.href = 'admin-dashboard.html';
            }
        });
    }
    ).catch(error => {
        // if the error is lowercase is query retured no rows
        if (error.toString().toLowerCase().includes('query returned no rows')) {
            errorMessage = 'Invalid admin username or password';
        }

        Swal.fire({
            title: 'Error!',
            text: error, // ensure error is a string
            icon: 'error',
            confirmButtonText: 'Ok'
        });
    });
});
