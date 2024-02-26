const invoke = window.__TAURI__.invoke
const notification = window.__TAURI__.notification

if (isAdminLoggedIn()) {
    const adminProfileForm = document.querySelector('#admin-profile-form');
    const adminId = localStorage.getItem('adminId');
    const adminUsername = localStorage.getItem('adminUsername');

    const adminUsernameInput = document.getElementById('admin-username-input');
    const adminNameInput = document.getElementById('admin-name-input')
    const adminEmailInput = document.getElementById('admin-email-input')
    const adminPhoneNoInput = document.getElementById('admin-phone-no-input')


    var adminName = null;
    var adminEmail = null;
    var adminPhoneNo = null;
    var adminDepartment = null;

    invoke('get_admin', {
        username: adminUsername,
    }).then(response => {
        let admin = response;
        adminName = admin.full_name;
        adminEmail = admin.email;
        adminPhoneNo = admin.phone_no;
        adminDepartment = admin.department;

        // set the values of the HTML elements
        document.getElementById('admin-username').innerHTML = `@${adminUsername}`;
        document.getElementById('admin-name').innerHTML = adminName;
        document.getElementById('admin-email').innerHTML = adminEmail;
        document.getElementById('admin-phone-no').innerHTML = adminPhoneNo;
        document.getElementById('admin-department').innerHTML = adminDepartment;

        // set the values for the input elements
        adminUsernameInput.value = adminUsername;
        adminNameInput.value = adminName;
        adminEmailInput.value = adminEmail;
        adminPhoneNoInput.value = adminPhoneNo;
    }
    ).catch(error => {
        notification.sendNotification({
            title: `Error!`,
            body: `${error}`, // ensure error is a string
        });

    })

}


if (isEmployeeLoggedIn()) {
    const employeeId = localStorage.getItem('employeeId');
    const employeeUsername = localStorage.getItem('employeeUsername');

    const employeeUsernameInput = document.getElementById('employee-username-input');
    const employeeNameInput = document.getElementById('employee-name-input')
    const employeeEmailInput = document.getElementById('employee-email-input')
    const employeePhoneNoInput = document.getElementById('employee-phone-no-input')


    var employeeName = null;
    var employeeEmail = null;
    var employeePhoneNo = null;
    var employeeDepartment = null;

    invoke('get_employee', {
        username: employeeUsername,
    }).then(response => {
        let employee = response;
        employeeName = employee.full_name;
        employeeEmail = employee.email;
        employeePhoneNo = employee.phone_no;
        employeeDepartment = employee.department;

        // set the values of the HTML elements
        document.getElementById('employee-username').innerHTML = `@${employeeUsername}`;
        document.getElementById('employee-name').innerHTML = employeeName;
        document.getElementById('employee-email').innerHTML = employeeEmail;
        document.getElementById('employee-phone-no').innerHTML = employeePhoneNo;
        document.getElementById('employee-department').innerHTML = employeeDepartment;

        // set the values for the input elements
        employeeUsernameInput.value = employeeUsername;
        employeeNameInput.value = employeeName;
        employeeEmailInput.value = employeeEmail;
        employeePhoneNoInput.value = employeePhoneNo;
    }
    ).catch(error => {
        notification.sendNotification({
            title: `Error!`,
            body: `${error}`, // ensure error is a string
        });

    })

}