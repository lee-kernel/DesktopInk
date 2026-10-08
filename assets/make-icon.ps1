# Render Lucide's type.svg geometry onto a blue rounded square.
# Source and full ISC/MIT notices are preserved next to this script.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$frames = @()
foreach ($size in @(16, 24, 32, 48, 64, 128, 256)) {
    $bitmap = [System.Drawing.Bitmap]::new($size, $size)
    $g = [System.Drawing.Graphics]::FromImage($bitmap)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.Clear([System.Drawing.Color]::Transparent)
    $g.ScaleTransform($size / 32.0, $size / 32.0)
    $background = [System.Drawing.Drawing2D.GraphicsPath]::new()
    $background.AddArc(1, 1, 10, 10, 180, 90)
    $background.AddArc(21, 1, 10, 10, 270, 90)
    $background.AddArc(21, 21, 10, 10, 0, 90)
    $background.AddArc(1, 21, 10, 10, 90, 90)
    $background.CloseFigure()
    $brush = [System.Drawing.SolidBrush]::new([System.Drawing.Color]::FromArgb(37, 99, 235))
    $g.FillPath($brush, $background)
    $g.TranslateTransform(4, 4)
    $pen = [System.Drawing.Pen]::new([System.Drawing.Color]::White, 2)
    $pen.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
    $pen.EndCap = [System.Drawing.Drawing2D.LineCap]::Round
    $pen.LineJoin = [System.Drawing.Drawing2D.LineJoin]::Round
    $g.DrawLine($pen, 12, 4, 12, 20)
    $top = [System.Drawing.Drawing2D.GraphicsPath]::new()
    $top.AddLine(4, 7, 4, 5)
    $top.AddArc(4, 4, 2, 2, 180, 90)
    $top.AddLine(5, 4, 19, 4)
    $top.AddArc(18, 4, 2, 2, 270, 90)
    $top.AddLine(20, 5, 20, 7)
    $g.DrawPath($pen, $top)
    $g.DrawLine($pen, 9, 20, 15, 20)
    $stream = [System.IO.MemoryStream]::new()
    $bitmap.Save($stream, [System.Drawing.Imaging.ImageFormat]::Png)
    $frames += ,@($size, $stream.ToArray())
    if ($size -eq 256) { $bitmap.Save((Join-Path $PSScriptRoot 'app-icon.png'), [System.Drawing.Imaging.ImageFormat]::Png) }
    $stream.Dispose(); $top.Dispose(); $pen.Dispose(); $brush.Dispose(); $background.Dispose(); $g.Dispose(); $bitmap.Dispose()
}
$output = [System.IO.File]::Create((Join-Path $PSScriptRoot 'app.ico'))
$writer = [System.IO.BinaryWriter]::new($output)
try {
    $writer.Write([uint16]0); $writer.Write([uint16]1); $writer.Write([uint16]$frames.Count)
    $offset = 6 + 16 * $frames.Count
    foreach ($frame in $frames) {
        $dimension = if ($frame[0] -eq 256) { 0 } else { $frame[0] }
        $writer.Write([byte]$dimension); $writer.Write([byte]$dimension)
        $writer.Write([byte]0); $writer.Write([byte]0)
        $writer.Write([uint16]1); $writer.Write([uint16]32)
        $writer.Write([uint32]$frame[1].Length); $writer.Write([uint32]$offset)
        $offset += $frame[1].Length
    }
    foreach ($frame in $frames) { $writer.Write([byte[]]$frame[1]) }
} finally { $writer.Dispose(); $output.Dispose() }
