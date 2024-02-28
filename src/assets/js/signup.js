// initialize the tauri API for invoking the rust functions and sending desktop notifications
const invoke = window.__TAURI__.invoke
const notification = window.__TAURI__.notification
const dialog = window.__TAURI__.dialog

// Regular expressions for validating user input
const usernamePattern = /^.{6,}$/;
const phonePattern = /^\d{10}$/;
const passwordPattern = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[!@#$*]).{8,}$/;

// intialized the signup form
const employeeForm = document.querySelector('#employee-signup-form');

// add functionality to the checkbox to enable the submit button
function checkAgreement(agreeCheckbox) {
    if (agreeCheckbox.checked) {
        document.querySelector('#employee-signup-submit').disabled = false;
    } else {
        document.querySelector('#employee-signup-submit').disabled = true;
    }
}

employeeForm.addEventListener('submit', (event) => {
    event.preventDefault();

    const employeeFullName = document.querySelector('#employee-signup-name').value;
    const employeeUsername = document.querySelector('#employee-signup-username').value;
    const employeeEmail = document.querySelector('#employee-signup-email').value;
    const employeePhoneNo = document.querySelector('#employee-signup-phone').value;
    const employeeDepartment = document.querySelector('#employee-signup-department').value;
    const employeePassword = document.querySelector('#employee-signup-password').value;
    const employeeConfirmPassword = document.querySelector('#employee-signup-confirm-password').value;

    if (employeePassword !== employeeConfirmPassword) {
        Swal.fire({
            title: 'Error!',
            text: 'Passwords do not match',
            icon: 'error',
            confirmButtonText: 'Ok'
        });
        // clear the password fields
        document.querySelector('#employee-signup-password').value = '';
        document.querySelector('#employee-signup-confirm-password').value = '';
        return;
    } else if (!usernamePattern.test(employeeUsername)) {
        // Show error message: "Username should be at least 6 characters long"
        Swal.fire({
            title: 'Error!',
            text: 'Username should be at least 6 characters long',
            icon: 'error',
            confirmButtonText: 'Ok'
        });
        return;
    } else if (!phonePattern.test(employeePhoneNo)) {
        // Show error message: "Phone number should be exactly 10 digits long"
        Swal.fire({
            title: 'Error!',
            text: 'Phone number should be exactly 10 digits long',
            icon: 'error',
            confirmButtonText: 'Ok'
        });
        return;
    } else if (!passwordPattern.test(employeePassword)) {
        // Show error message: "Password should be at least 8 characters long, 
        // have at least one uppercase letter, one lowercase letter, one number, 
        // and one special character"
        Swal.fire({
            title: 'Password is not strong enough!',
            html: `
            <ul>
            <li>Be at least 8 characters long</li>
            <li>Have at least one uppercase letter</li>
            <li>Have at least one lowercase letter</li>
            <li>Have at least one number</li>
            <li>Have at least one special character (like !, @, #, $,*)</li>
           </ul>`,
            icon: 'error',
            confirmButtonText: 'Ok'
        });
        return;
    }


    invoke('create_employee', {
        fullname: employeeFullName,
        username: employeeUsername,
        email: employeeEmail,
        phone: employeePhoneNo,
        department: employeeDepartment,
        password: employeePassword,
    }).then(response => {
        Swal.fire({
            title: "Signup Successful!",
            text: "Do you want to login now?",
            showDenyButton: true,
            showCancelButton: false,
            confirmButtonText: `Yes`,
            denyButtonText: `No`,
        }).then((result) => {
            /* Read more about isConfirmed, isDenied below */
            if (result.isConfirmed) {
                window.location.href = "login.html";
            } else if (result.isDenied) {
                employeeForm.reset();
            }
        });
    }
    ).catch(error => {
        // if error occurs output appropriate error message
        if (error.toString() === "UNIQUE constraint failed: employees.phone") {
            errorMessage = "Phone number already used by another user";
        } else if (error.toString() === "UNIQUE constraint failed: employees.email") {
            errorMessage = "Email already used by another user";
        }
        else if (error.toString() === "UNIQUE constraint failed: employees.username") {
            errorMessage = "Username already used by another user";
        }
        else {
            errorMessage = `${error}`;
        }

        Swal.fire({
            title: 'Error!',
            text: `${errorMessage}`,
            icon: 'error',
            confirmButtonText: 'Ok'
        });
    });
});

// write a script that invokes the get detpartments command when the page loads and populates the select element with the departments
invoke('get_departments').then(response => {
    const departments = response;
    const select = document.querySelector('#employee-signup-department');
    departments.forEach(department => {
        const option = document.createElement('option');
        option.value = department.department_name;
        option.textContent = department.department_name;
        select.appendChild(option);
    });
}).catch(error => {
    console.error(error);
});