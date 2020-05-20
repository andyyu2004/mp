fn stuff() {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = File::open("Aerosmith - What It Takes.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.set_volume(0.1);
    sink.append(source);

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    println!("{}", input);
}
