var backRequests = 'http://localhost:8080';

module.exports = {
    "/api": {
        "target": backRequests,
        secure: false,
        changeOrigin: true,
    }
}