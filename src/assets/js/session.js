// function to check if employee is logged in
function loginEmployee(employeeId, employeeUsername, employeeFullName) {
    localStorage.setItem('employeeId', `${employeeId}`);
    localStorage.setItem('employeeUsername', `${employeeUsername}`);
    localStorage.setItem('employeeName', `${employeeFullName}`);
}

// function to check if admin is logged in
function loginAdmin(adminId, adminUsername, adminFullName) {
    localStorage.setItem('adminId', `${adminId}`);
    localStorage.setItem('adminUsername', `${adminUsername}`);
    localStorage.setItem('adminName', `${adminFullName}`);
}

// function to logout employee 
function logoutEmployee() {
    localStorage.removeItem('employeeId');
    localStorage.removeItem('employeeUsername');
    localStorage.removeItem('employeeName');
    window.location.href = 'index.html';
}

// function to logout admin
function logoutAdmin() {
    localStorage.removeItem('adminId');
    localStorage.removeItem('adminUsername');
    localStorage.removeItem('adminName');
    window.location.href = 'index.html';
}

// function to check if employee is logged in
function isEmployeeLoggedIn() {
    let EmployeeLoggedIn = localStorage.getItem('employeeId') !== null;
    return EmployeeLoggedIn;
}

// function to check if admin is logged in
function isAdminLoggedIn() {
    let AdminLoggedIn = localStorage.getItem('adminId') !== null;
    return AdminLoggedIn;
}
