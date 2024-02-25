const invoke = window.__TAURI__.invoke
const notification = window.__TAURI__.notification

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
        Swal.fire({
            title: `Employee logged in successfully!`,
            // text: JSON.stringify(response), // ensure response is a string
            icon: 'success',
            confirmButtonText: 'Ok'
        });
    }
    ).catch(error => {
        // if the rror i lowercase is query retured no rows
        if (error.toString().toLowerCase().includes('query returned no rows')) {
            errorMessage = 'Invalid employee username or password';
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
        Swal.fire({
            title: `Admin logged in successfully!`,
            // text: JSON.stringify(response), // ensure response is a string
            icon: 'success',
            confirmButtonText: 'Ok'
        });
    }
    ).catch(error => {
        // if the rror i lowercase is query retured no rows
        if (error.toString().toLowerCase().includes('query returned no rows')) {
            errorMessage = 'Invalid admin username or password';
        }

        Swal.fire({
            title: 'Error!',
            text: errorMessage, // ensure error is a string
            icon: 'error',
            confirmButtonText: 'Ok'
        });
    });
});
