// external libraries
extern crate piston_window;
extern crate find_folder;

// name space
use piston_window::*;

fn main( ) {
	// create window object called `window`
	let mut window: PistonWindow = WindowSettings::new (
		"piston: hello_world", // window name
		[ 1920, 1080 ]           // window size
	)
	.exit_on_esc( true )    // exit the program when esc is pressed
	.fullscreen( true )     // enables full screen
	.build( )               // builds the piston window frow the window settings
	.unwrap( );             // panics on error

	window.set_capture_cursor( true ); // captures the cursor on the screen
	                                   // set here because window settings doesn't have
	                                   // capture cursor option

	// search for folder "assets"
	let assets = find_folder::Search::ParentsThenKids( 3, 3 )
		.for_folder( "assets" ).unwrap( );
	println!( "{:?}", assets );

	let mut glyphs = window.load_font( assets.join( "FiraSans-Regular.ttf" ) ).unwrap( );

	window.set_lazy( true );

	while let Some( e ) = window.next( ) {
		// draw 2d in window
		// e is an event in the window
		// c is the window context
		// g is the graphics resource
		// device is teh rendering device
		window.draw_2d( &e, | c, g, device | {
			// sets the absolute location of the text in pixels
			let transform = c.transform.trans( 10.0, 100.0 );

			// clears the screen and makes it black
			clear( [ 0.0, 0.0, 0.0, 1.0 ], g );

			// draws text in green color
			text::Text::new_color( [ 0.0, 1.0, 0.0, 1.0 ], 32 ).draw(
				"Hello world.", // text to draw
				&mut glyphs,    // font to use
				&c.draw_state,  // ?
				transform, g    // location to print text and graphics resource
			).unwrap( );

			// Update glyphs before rendering
			glyphs.factory.encoder.flush( device );
		} );
	}

}
