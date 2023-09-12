import 'package:flutter/material.dart';
import 'package:rust_stuff/screens/donations_page.dart';

import 'InitialPage.dart';

class Base extends StatefulWidget {
  const Base({super.key});

  @override
  State<Base> createState() => _BaseState();
}

class _BaseState extends State<Base> {
  final PageController _pageController = PageController();
  bool donor = true;

  setDonor(bool ans){
    donor = ans;
  }
  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;
    return Scaffold(
      backgroundColor: Colors.white,
      body: SingleChildScrollView(
        physics: const NeverScrollableScrollPhysics(),
        child: SizedBox(
          height: size.height,
          child: PageView(
            physics: const NeverScrollableScrollPhysics(),
            controller: _pageController,
            children: [
              Initial(controller: _pageController, setDonor: setDonor,),
              DonationPage(controller: _pageController, isDonor: donor,),

            ],
          ),
        ),
      ),
    );
  }
}
