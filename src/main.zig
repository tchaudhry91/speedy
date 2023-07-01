const std = @import("std");

const Server = struct {
    stream_server: std.net.StreamServer,

    fn init(addr: []const u8, port: u16) !Server {
        var stream_server = std.net.StreamServer.init(.{});
        const listen_address = try std.net.Address.parseIp4(addr, port);
        try stream_server.listen(listen_address);
        return Server{ .stream_server = stream_server };
    }

    pub fn deinit(self: *Server) void {
        self.stream_server.deinit();
    }

    pub fn accept(self: *Server) !void {
        const conn = try self.stream_server.accept();
        defer conn.stream.close();

        var start = try std.time.Timer.start();

        var buf: [1024]u8 = std.mem.zeroes([1024]u8);
        var written: usize = 0;
        var i: u32 = 0;
        // Write 100 MB
        while (i < 102400) : (i += 1) {
            const written_pass = try conn.stream.write(&buf);
            written += written_pass;
        }
        std.debug.print("Written: {d}MBytes. Client: {any}. Took: {d}ms.\n", .{ written / (1024 * 1024), conn.address.in, start.read() / 1000000 });
    }
};

pub fn main() !void {
    var server = try Server.init("127.0.0.1", 8567);
    defer server.deinit();
    while (true) {
        try server.accept();
    }
}
